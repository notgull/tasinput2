/*
 * src/lib.rs
 *
 * tasinput2 - Input plugin for generating tool assisted speedruns
 * Copyright (C) 2020 not_a_seagull
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

mod plugin_info;

use plugin_info::PluginInfo;
use std::{
    ffi::{c_void, CString},
    ptr,
};

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DllMain() {}

/// Take a PluginInfo struct and put this plugin's information into it.
///
/// # Safety
///
/// This is called from a C function, safety is unfortunately not a priority.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn GetDllInfo(plugin: *mut PluginInfo) {
    // set version and type
    (*plugin).version = 0x0200;
    (*plugin).plugin_type = 4;

    // set plugin name
    let plugin_name = format!("TAS Input Plugin v{}", env!("CARGO_PKG_VERSION"));

    // convert the plugin name to the equivalent C string
    let c_plugin_name = CString::new(plugin_name).unwrap_or_else(|e| {
        let mut v = e.into_vec();
        v.retain(|x| *x != 0);
        CString::new(v).unwrap() // note: we really shouldn't unwrap in C code, but then again there should be no possible way that this fails
    });

    // copy over the new bytes
    ptr::copy_nonoverlapping(
        c_plugin_name.as_ptr(),
        (*plugin).name,
        c_plugin_name.to_bytes_with_nul().len(),
    );
}

#[link(name = "guiadapter", kind = "static")]
extern "C" {
    fn show_about_box(parent_ptr: *const c_void);
}

/// Create a message box that shows a dialog box.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DllAbout(parent: *const c_void) {
    show_about_box(parent);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
