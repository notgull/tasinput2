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

extern crate qt_widgets;
extern crate thiserror;

mod controller;
mod gui;
mod inputs;
mod plugin_info;

use plugin_info::PluginInfo;
use std::{
    ffi::{c_void, CString},
    ptr,
};

pub use controller::*;

// the only safe part of the dll info: parsing the string
#[cold]
fn get_version_string() -> CString {
    let plugin_name = format!("TAS Input Plugin 2 v{}", env!("CARGO_PKG_VERSION"));

    // convert plugin name to a cstring
    CString::new(plugin_name).unwrap_or_else(|e| {
        let mut v = e.into_vec();
        v.retain(|x| *x != 0);
        CString::new(v).unwrap() // this shouldn't throw an error
    })
}

/// Put the DLL's information into a plugin information struct.
///
/// # Safety
///
/// This is called from C code, it is already unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn GetDllInfo(plugin_info: *mut PluginInfo) {
    // an increment over the past version
    (*plugin_info).version = 0x0200;
    // indicate this is a controller plugin
    (*plugin_info).plugin_type = 4;
    // copy the version string into the plugin name
    let version = get_version_string();
    ptr::copy_nonoverlapping(
        version.as_ptr(),
        (*plugin_info).name,
        version.to_bytes_with_nul().len(),
    );
}

/// Initialize the DLL.
///
/// # Safety
///
/// This is called from C code, it is already unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DllMain(
    _parent_instance: *const c_void,
    launch_reason: i32,
    _reserved: *const c_void,
) -> i32 {
    match launch_reason {
        1 /* DLL_PROCESS_ATTACH */ => gui::start_application(),
        0 /* DLL_PROCESS_DETACH */ => gui::close_application(),
        _ => { /* noop */ }
    }

    1
}

/// Close this DLL.
///
/// # Safety
///
/// Exclusively called from C code
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CloseDll() {
    gui::close_application();
}

/// Process raw data sent to this controller. This is a No-op in the original TAS plugin and it is a no-op here too.
///
/// # Safety
///
/// This function does nothing that is unsafe, because it does nothing.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ControllerCommand(_controllerNumber: i32, _data_pointer: *const u8) {}
