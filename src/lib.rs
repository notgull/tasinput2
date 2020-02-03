/*
 * src/lib.rs
 * tasinput2 - Plugin for creating TAS inputs
 *
 * This file is part of tasinput2.
 *
 * tasinput2 is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * tasinput2 is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with tasinput2.  If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate lazy_static;

mod controller;
#[doc(hidden)]
#[macro_use]
pub mod debug;
mod inputs;
mod state;

use std::{
    ffi::{c_void, CString},
    os::raw::c_char,
    panic::catch_unwind,
    sync::{
        atomic::{AtomicBool, AtomicPtr},
        Arc, Mutex,
    },
};

pub use controller::*;
pub use inputs::{Directional, Inputs};
pub use state::Tasinput2State;
use std::convert::TryInto;

pub const CONTROLLER_COUNT: u32 = 4;

lazy_static! {
    static ref IS_INITIALIZED: Arc<Mutex<AtomicBool>> =
        Arc::new(Mutex::new(AtomicBool::new(false)));
    pub static ref STATE: Arc<Mutex<Tasinput2State>> = Arc::new(Mutex::new(Tasinput2State::new()));
}

// the only safe part of the dll info: parsing the string
#[cold]
fn get_version_string() -> CString {
    let plugin_name = concat!("TAS Input Plugin 2 v", env!("CARGO_PKG_VERSION"));

    // convert plugin name to a cstring
    CString::new(plugin_name).unwrap_or_else(|e| {
        let mut v = e.into_vec();
        v.retain(|x| *x != 0);
        CString::new(v).unwrap() // this shouldn't throw an error
    })
}

/// Start up this plugin.
///
/// # Safety
///
/// Exclusively called from C code
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn PluginStartup(
    core_lib_handle: m64p_sys::m64p_dynlib_handle,
    context: *mut c_void,
    debug_callback: unsafe extern "C" fn(*mut c_void, m64p_sys::m64p_msg_level, *const c_char),
) -> m64p_sys::m64p_error {
    match catch_unwind(|| {
        let mut lock = match IS_INITIALIZED.lock() {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Initialization error: {}", e);
                return m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL;
            }
        };

        *(match debug::DEBUG_OUT.lock() {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Unable to acquire debug lock: {}", e);
                return m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL;
            }
        }) = Some(debug::Debugger {
            debug_fn: debug_callback,
        });

        let is_started: &mut bool = lock.get_mut();
        if *is_started {
            return m64p_sys::m64p_error_M64ERR_ALREADY_INIT;
        }
        *is_started = true;

        let mut lock = match STATE.lock() {
            Ok(l) => l,
            Err(e) => {
                dprintln!("Mutex error: {}", e);
                return m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL;
            }
        };
        lock.context = Some(AtomicPtr::new(context));

        if let Err(e) = lock.start_qt() {
            dprintln!("Unable to start QT: {:?}", e);
            return m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL;
        };

        0
    }) {
        Ok(_) => m64p_sys::m64p_error_M64ERR_SUCCESS,
        Err(e) => {
            eprintln!("Panic occurred: {:?}", e);
            m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL
        }
    }
}

/// Put the DLL's information into a plugin information struct.
///
/// # Safety
///
/// This is called from C code, it is already unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn PluginGetVersion(
    plugin_type: *mut m64p_sys::m64p_plugin_type,
    plugin_version: *mut i32,
    api_version: *mut i32,
    plugin_name: *mut *const c_char,
    capabilities: *mut i32,
) -> m64p_sys::m64p_error {
    match catch_unwind(|| {
        // an increment over the past version
        if !plugin_version.is_null() {
            *plugin_version = 0x0200;
        }

        // indicate this is a controller plugin
        if !plugin_type.is_null() {
            *plugin_type = m64p_sys::m64p_plugin_type_M64PLUGIN_INPUT;
        }

        // indicate the API version this expects
        if !api_version.is_null() {
            *api_version = 0x020100;
        }

        // what capabilities does this plugin have?
        if !capabilities.is_null() {
            *capabilities = 0;
        }

        // copy the version string into the plugin name
        if !plugin_name.is_null() {
            let version = get_version_string();
            *plugin_name = version.into_raw();
        };

        0
    }) {
        Ok(_) => m64p_sys::m64p_error_M64ERR_SUCCESS,
        Err(e) => {
            dprintln!("Panic occurred: {:?}", e);
            m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL
        }
    }
}

/// Close this DLL.
///
/// # Safety
///
/// Exclusively called from C code
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn PluginShutdown() -> m64p_sys::m64p_error {
    match catch_unwind(|| {
        let mut lock = match IS_INITIALIZED.lock() {
            Ok(l) => l,
            Err(e) => {
                dprintln!("{:?}", e);
                return m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL;
            }
        };
        let is_started: &mut bool = lock.get_mut();
        if !(*is_started) {
            return m64p_sys::m64p_error_M64ERR_NOT_INIT;
        }

        if let Err(e) = (*(match STATE.lock() {
            Ok(l) => l,
            Err(e) => {
                dprintln!("{:?}", e);
                return m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL;
            }
        }))
        .end()
        {
            eprintln!("tasinput2 error: {:?}", e);
            return m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL;
        }

        *is_started = false;

        0
    }) {
        Ok(_) => m64p_sys::m64p_error_M64ERR_SUCCESS,
        Err(e) => {
            eprintln!("Panic occurred: {:?}", e);
            m64p_sys::m64p_error_M64ERR_SYSTEM_FAIL
        }
    }
}

/// Process raw data sent to this controller.
///
/// # Safety
///
/// This function does nothing that is unsafe, because it does nothing.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ControllerCommand(_controllerNumber: i32, _data_pointer: *mut c_char) {}

/// Read raw data from a controller.
///
/// # Safety
///
/// This function does not do anything.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ReadController(_controller_number: i32, _data_pointer: *mut c_char) {}

const WORKING_CTRL_COUNT: usize = 1;

/// Initialize the set of controllers
fn initialize_controllers() {
    let state = STATE.lock().unwrap();
    for i in 0usize..WORKING_CTRL_COUNT {
        if let Err(e) = state.start_controller(i) {
            dprintln!("Error initializing controller: {:?}", e);
        }
    }
}

/// Initialize a controller.
///
/// # Safety
///
/// This function is exclusively called from C code.
///
/// # TODO
///
/// Capture input.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn InitiateControllers(_controller_info: m64p_sys::CONTROL_INFO) {
    match catch_unwind(|| {
        initialize_controllers();

        0
    }) {
        Ok(_) => {}
        Err(e) => dprintln!("Unable to initiate controllers: {:?}", e),
    };
}

/// Called when a ROM is open.
///
/// # Safety
///
/// This functions is called exclusively from C code.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn RomOpen() -> i32 {
    match catch_unwind(|| {
        initialize_controllers();
        *STATE.lock().unwrap().romopen.get_mut() = true;

        0
    }) {
        Ok(_) => 1,
        Err(e) => {
            dprintln!("Rom open failed: {:?}", e);
            0
        }
    }
}

/// Called when the ROM is closed
///
/// # Safety
///
/// This functions is called exclusively from C code.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn RomClosed() -> i32 {
    match catch_unwind(|| {
        let mut state = STATE.lock().unwrap();
        for i in 1usize..WORKING_CTRL_COUNT {
            if let Err(e) = state.stop_controller(i) {
                dprintln!("Unable to stop controller thread: {}", e);
            }
        }
        *state.romopen.get_mut() = false;

        0
    }) {
        Ok(_) => 1,
        Err(e) => {
            dprintln!("Rom close failed: {:?}", e);
            0
        }
    }
}

/// Pass an SDL signal through to the input.
///
/// # Safety
///
/// This function is called exclusively from C code.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn SDL_KeyDown(_keymod: i32, _keysym: i32) {}

/// Pass an SDL signal through to the input.
///
/// # Safety
///
/// This function is called exclusively from C code.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn SDL_KeyUp(_keymod: i32, _keysym: i32) {}

/// Pass the buttons into the emulator
///
/// # Safety
///
/// This functions is called exclusively from C code.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn GetKeys(controller_num: i32, output: *mut m64p_sys::BUTTONS) {
    match catch_unwind(|| {
        let state = STATE.lock().unwrap();
        let index = match controller_num.try_into() {
            Ok(i) => i,
            Err(e) => {
                dprintln!("Invalid index: {:?}", e);
                return;
            }
        };

        let buttons: Inputs = match state.get_inputs(index) {
            Ok(i) => i,
            Err(e) => {
                dprintln!("Unable to retrieve inputs: {:?}", e);
                return;
            }
        };
        let buttons = buttons.to_canonical();

        (*output)._bitfield_1 = buttons._bitfield_1;
    }) {
        Ok(_) => { },
        Err(e) => dprintln!("Unable to get keys: {:?}", e)
    }
}
