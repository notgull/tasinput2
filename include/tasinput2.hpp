/*
 * include/tasinput2.hpp
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

#ifndef TASINPUT2_HPP
#define TASINPUT2_HPP

#include "m64p/m64p_types.h"
#include "m64p/m64p_plugin.h"
#include <mutex>

extern "C" {
typedef void (*DebugCallback)(void *, int, const char *);

// fill out info regarding the plugin
EXPORT m64p_error PluginGetVersion(m64p_plugin_type* plugin_type, int* plugin_version,
                            int* api_version, const char** plugin_name,
                            int* capabilities);
// start up the plugin
EXPORT m64p_error PluginStartup(m64p_dynlib_handle core_handle, void *d_context, DebugCallback d_callback);

// shut down the plugin
EXPORT m64p_error PluginShutdown();

// some no-op functions
EXPORT void ControllerCommand(int ctrl_number, char *data_ptr);
EXPORT void ReadController(int ctrl_number, char *data_ptr);
EXPORT void SDL_KeyDown(int keymod, int keysym);
EXPORT void SDL_KeyUp(int keymod, int keysym);

// init controllers
EXPORT void InitiateControllers(CONTROL_INFO controller_info);

// open/close rom
EXPORT int RomOpen();
EXPORT int RomCloseD();
}

#endif
