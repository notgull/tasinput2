/*
 * src/tasinput2.c
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

#include "plugin.h"
#include "tasinput2.h"
#include "version.h"

#include <pthread.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "m64p/m64p_plugin.h"
#include "m64p/m64p_types.h"

bool is_init = false;
bool is_rom_open = false;
DebugCallback debug_callback;
void* debug_context;

// printing messages via the debug callback
void debug_printf(int level, const char *format, ...) {
  char buffer[1024];
  va_list args;

  // set up the VA list
  va_start(args, format);
  
  // if the debug callback is null, just print to stderr
  if (!debug_callback) {
    vfprintf(stderr, format, args);
    return;
  }

  // otherwise, print to the buffer
  vsprintf(buffer, format, args);
 
  // call the debug callback with the buffer
  (*debug_callback)(debug_context, level, buffer);

  va_end(args);
}

// get a string containing the version information
char* get_version() {
  const char* version_template = "TAS Input Plugin 2 by not_a_seagull";
  char* res = (char*)malloc(sizeof(char) * (strlen(version_template) + 1));
  strncpy(res, version_template, strlen(version_template));
  return res;
}

// set up the plugin information
EXPORT m64p_error PluginGetVersion(m64p_plugin_type* plugin_type,
                                   int* plugin_version, int* api_version,
                                   const char** plugin_name,
                                   int* capabilities) {
  // we are an input plugin
  if (plugin_type != NULL) {
    *plugin_type = M64PLUGIN_INPUT;
  }

  // plugin version
  if (plugin_version != NULL) {
    *plugin_version = 0x10001;
  }

  // api version we are using
  if (api_version != NULL) {
    *api_version = 0x00020100;
  }

  // the capabilities of our plugin
  if (capabilities != NULL) {
    *capabilities = 0;
  }

  // the name of our plugin
  if (plugin_name != NULL) {
    *plugin_name = get_version();
  }

  return M64ERR_SUCCESS;
}

// start up the plugin
EXPORT m64p_error PluginStartup(m64p_dynlib_handle core_handle, void* d_context,
                                DebugCallback d_callback) {
  lock_mutex();

  // if we're already initialized, throw an error
  if (is_init) {
    unlock_mutex();
    return M64ERR_ALREADY_INIT;
  }

  is_init = true;
  debug_context = d_context;
  debug_callback = d_callback;

  // TODO: corelib funcs
  // TODO: configuration and etc.

  unlock_mutex();
  return M64ERR_SUCCESS;
}

// shut down the plugin
EXPORT m64p_error PluginShutdown() {
  lock_mutex();

  if (!is_init) {
    unlock_mutex();
    return M64ERR_NOT_INIT;
  }

  is_init = false;
  debug_context = NULL;
  debug_callback = NULL;

  // TODO: corelib: set funcs to null

  unlock_mutex();
  return M64ERR_SUCCESS;
}

// some no-op functions
EXPORT void ControllerCommand(int ctrl_number, char* data_ptr) {}
EXPORT void ReadController(int ctrl_number, char* data_ptr) {}
EXPORT void SDL_KeyDown(int keymod, int keysym) {}
EXPORT void SDL_KeyUp(int keymod, int keysym) {}

// initialize the controllers and open the QT windows
EXPORT void InitiateControllers(CONTROL_INFO controller_info) {
  lock_mutex();
  const uint32_t CONTROLLER_COUNT = 1;  // TODO: not this

  launch_controllers(CONTROLLER_COUNT);

  // todo: with configuration
  controller_info.Controls->Present = 1;

  unlock_mutex();
}

// set the rom flag to open
EXPORT int RomOpen() {
  lock_mutex();
  is_rom_open = true;
  unlock_mutex();

  return 0;
}

// set the rom flag to closed and destroy the controllers
EXPORT int RomClosed() {
  lock_mutex();
  is_rom_open = false;

  // destroy the application and merge the thread
  deinit_controllers();  
 
  unlock_mutex();
  
  return 0;
}

// transmit the buttons being pressed to the core
EXPORT void GetKeys(int ctrl_number, BUTTONS* output) {
  lock_mutex();
  *output = get_ctrl_keys(ctrl_number);
  unlock_mutex();
}
