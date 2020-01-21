/*
 * src/plugin_info.rs
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

use std::os::raw::c_char;

/// The information required of a plugin.
#[repr(C)]
pub struct PluginInfo {
    /// The version of the plugin that we are using.
    pub version: u16,
    /// The type of plugin (should always be 4).
    pub plugin_type: u16,
    /// The name of the plugin
    pub name: *mut c_char,
    // reserved values, BOOLs are i32's
    reserved1: i32,
    reserved2: i32,
}
