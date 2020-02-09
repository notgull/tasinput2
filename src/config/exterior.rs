/*
 * src/config/exterior.rs
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

//! Exterior functions for loading code configuration.

use std::{
    ffi::CString,
    os::raw::c_char,
};

// load a dynamic procedure on linux
#[cfg(not(windows))]
unsafe fn load_dynamic_procedure(
    core_lib_handle: m64p_sys::m64p_dynlib_handle,
    proc_name: *const c_char,
) -> m64p_sys::m64p_function {
    // a binding to dlsym is contained in the libc crate
    Some(libc::dlsym(core_lib_handle, proc_name))
}

// load a dynamic procedure on win32
#[cfg(windows)]
unsafe fn load_dynamic_procedure(
    core_lib_handle: m64p_sys::m64p_dynlib_handle,
    proc_name: *const c_char,
) -> m64p_sys::m64p_function {
    // a binding to GetProcAddress is contained in the winapi crate
    Some(winapi::um::libloaderapi::GetProcAddress(core_lib_handle, proc_name))
}

// load a dynamic procedure
fn load_dynamic_lib(
    core_lib_handle: m64p_sys::m64p_dynlib_handle, 
    lib_name: &'static str
) -> m64p_sys::m64p_function {
    // convert lib_name to a CString
    // note: since this string is static, let's assume that it is valid
    let lib_name = CString::new(lib_name).unwrap();

    // now convert it to a pointer
    let lib_name = lib_name.into_raw();

    unsafe { load_dynamic_procedure(core_lib_handle, lib_name) }
}

/// A reference to exterior functions required for configuration.
pub struct ConfigureFunctions {
    pub config_list_sections: m64p_sys::ptr_ConfigListSections,
    pub config_open_section: m64p_sys::ptr_ConfigOpenSection,
    pub config_delete_section: m64p_sys::ptr_ConfigDeleteSection,
    pub config_list_parameters: m64p_sys::ptr_ConfigListParameters,
    pub config_set_parameter: m64p_sys::ptr_ConfigSetParameter,
    pub config_get_parameter: m64p_sys::ptr_ConfigGetParameter,
    pub config_get_parameter_help: m64p_sys::ptr_ConfigGetParameterHelp,
    pub config_set_default_int: m64p_sys::ptr_ConfigSetDefaultInt,
    pub config_set_default_float: m64p_sys::ptr_ConfigSetDefaultFloat,
    pub config_set_default_bool: m64p_sys::ptr_ConfigSetDefaultBool,
    pub config_set_default_string: m64p_sys::ptr_ConfigSetDefaultString,
    pub config_get_param_int: m64p_sys::ptr_ConfigGetParamInt,
    pub config_get_param_float: m64p_sys::ptr_ConfigGetParamFloat,
    pub config_get_param_bool: m64p_sys::ptr_ConfigGetParamBool,
    pub config_get_param_string: m64p_sys::ptr_ConfigGetParamString,
}

impl ConfigureFunctions {
    /// Load all needed functions from the core library.
    pub fn new(lib: m64p_sys::m64p_dynlib_handle) -> ConfigureFunctions {
        ConfigureFunctions {
            config_list_sections:      load_dynamic_lib(lib, "ConfigListSections")     as m64p_sys::ptr_ConfigListSections,
            config_open_section:       load_dynamic_lib(lib, "ConfigOpenSection")      as m64p_sys::ptr_ConfigOpenSection,
            config_delete_section:     load_dynamic_lib(lib, "ConfigDeleteSection")    as m64p_sys::ptr_ConfigDeleteSection,
            config_list_parameters:    load_dynamic_lib(lib, "ConfigListParameters")   as m64p_sys::ptr_ConfigListParameters,
            config_set_parameter:      load_dynamic_lib(lib, "ConfigSetParameter")     as m64p_sys::ptr_ConfigSetParameter,
            config_get_parameter:      load_dynamic_lib(lib, "ConfigGetParameter")     as m64p_sys::ptr_ConfigGetParameter,
            config_get_parameter_help: load_dynamic_lib(lib, "ConfigGetParameterHelp") as m64p_sys::ptr_ConfigGetParameterHelp,
            config_set_default_int:    load_dynamic_lib(lib, "ConfigSetDefaultInt")    as m64p_sys::ptr_ConfigSetDefaultInt,
            config_set_default_float:  load_dynamic_lib(lib, "ConfigSetDefaultFloat")  as m64p_sys::ptr_ConfigSetDefaultFloat,
            config_set_default_bool:   load_dynamic_lib(lib, "ConfigSetDefaultBool")   as m64p_sys::ptr_ConfigSetDefaultBool,
            config_set_default_string: load_dynamic_lib(lib, "ConfigSetDefaultString") as m64p_sys::ptr_ConfigSetDefaultString,
            config_get_param_int:      load_dynamic_lib(lib, "ConfigGetParamInt")      as m64p_sys::ptr_ConfigGetParamInt,
            config_get_param_float:    load_dynamic_lib(lib, "ConfigGetParamFloat")    as m64p_sys::ptr_ConfigGetParamFloat,
            config_get_param_bool:     load_dynamic_lib(lib, "ConfigGetParamBool")     as m64p_sys::ptr_ConfigGetParamBool,
            config_get_param_string:   load_dynamic_lib(lib, "ConfigGetParamString")   as m64p_sys::ptr_ConfigGetParamString,
        }
    }
}
