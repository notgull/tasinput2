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

#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate qt_widgets;
extern crate thiserror;

mod controller;
mod gui;
mod inputs;
mod m64compat;
mod plugin_info;
mod state;

use plugin_info::PluginInfo;
use std::{
    ffi::{c_void, CString},
    ptr,
    sync::{Arc, Mutex},
};

pub use controller::*;
pub use state::Tasinput2State;
use std::os::raw::c_char;
use std::ptr::null_mut;

pub const CONTROLLER_COUNT: u32 = 4;

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
pub unsafe extern "C" fn PluginGetVersion(
    plugin_type: *mut m64compat::m64p_plugin_type,
    plugin_version: *mut i32,
    api_version: *mut i32,
    plugin_name: *mut *const c_char,
    capabilities: *mut i32,
) -> m64compat::m64p_error {
    // an increment over the past version
    if plugin_version.is_null() {
        *plugin_version = 0x0200;
    }

    // indicate this is a controller plugin
    if plugin_type.is_null() {
        *plugin_type = m64compat::m64p_plugin_type_M64PLUGIN_INPUT;
    }

    // indicate the API version this expects
    if api_version.is_null() {
        *api_version = 0x020100;
    }

    // what capabilities does this plugin have?
    if capabilities.is_null() {
        *capabilities = 0;
    }

    // copy the version string into the plugin name
    if plugin_name.is_null() {
        let version = get_version_string();
        *plugin_name = version.as_ptr();
    }

    m64compat::m64p_error_M64ERR_SUCCESS
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
