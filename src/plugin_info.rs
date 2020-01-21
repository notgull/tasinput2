/*
 * src/plugin_info.rs
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

use std::os::raw::c_char;

/// Represents plugin information, a structure passed into the library to acquire information regarding the plugin.
#[repr(C)]
pub struct PluginInfo {
    /// The version of the plugin. This will be set to 0x0200, an increment from tasinput1.
    pub version: u16,
    /// The type of plugin that this is. Should be set to PLUGIN_TYPE_CONTROLLER
    pub plugin_type: u16,
    /// The name of this library. Represents a C-level string
    pub name: *mut c_char,
    // reserved keywords.
    // note: these are BOOLs, which are signed ints in minwindef.h
    reserved1: i32,
    reserved2: i32,
}
