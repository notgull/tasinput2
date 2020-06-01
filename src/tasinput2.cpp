/*
 * src/tasinput2.cpp
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

#include "controller.hpp"
#include "tasinput2.hpp"
#include "version.hpp"

#include "m64p/m64p_plugin.h"
#include "m64p/m64p_types.h"

#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <mutex>
#include <thread>
#include <optional>

#include <QApplication>
#include <QCoreApplication>

// prevent threads from getting out of hand
static std::mutex lock;

std::mutex& get_lock() {
    return lock;
}

bool is_init = false;
bool is_rom_open = false;
DebugCallback debug_callback;
void* debug_context;

std::optional<std::thread> qt_thread = std::nullopt;
Controller *controllers[4] = {nullptr, nullptr, nullptr, nullptr};

// get a string containing the version information
char* get_version() {
  const char* version_template = "TAS Input Plugin 2 v%s by not_a_seagull";
  char* res = (char*)malloc(sizeof(char) * ((strlen(version_template) - 2) +
                                            strlen(TI2_VERSION) + 1));
  sprintf(res, version_template, TI2_VERSION);
  return res;
}

// set up the plugin information
EXPORT m64p_error PluginGetVersion(m64p_plugin_type* plugin_type,
                                   int* plugin_version, int* api_version,
                                   const char** plugin_name,
                                   int* capabilities) {
  // we are an input plugin
  if (plugin_type != nullptr) {
    *plugin_type = M64PLUGIN_INPUT;
  }

  // plugin version
  if (plugin_version != nullptr) {
    *plugin_version = 0x10001;
  }

  // api version we are using
  if (api_version != nullptr) {
    *api_version = 0x00020100;
  }

  // the capabilities of our plugin
  if (capabilities != nullptr) {
    *capabilities = 0;
  }

  // the name of our plugin
  if (plugin_name != nullptr) {
    *plugin_name = get_version();
  }

  return M64ERR_SUCCESS;
}

// start up the plugin
EXPORT m64p_error PluginStartup(m64p_dynlib_handle core_handle, void* d_context,
                                DebugCallback d_callback) {
  lock.lock();

  // if we're already initialized, throw an error
  if (is_init) {
    lock.unlock();
    return M64ERR_ALREADY_INIT;
  }

  is_init = true;
  debug_context = d_context;
  debug_callback = d_callback;

  // TODO: corelib funcs
  // TODO: configuration and etc.

  lock.unlock();
  return M64ERR_SUCCESS;
}

// shut down the plugin
EXPORT m64p_error PluginShutdown() {
  lock.lock();

  if (!is_init) {
    lock.unlock();
    return M64ERR_NOT_INIT;
  }

  is_init = false;
  debug_context = nullptr;
  debug_callback = nullptr;

  // TODO: corelib: set funcs to null

  lock.unlock();
  return M64ERR_SUCCESS;
}

// some no-op functions
EXPORT void ControllerCommand(int ctrl_number, char* data_ptr) {}
EXPORT void ReadController(int ctrl_number, char* data_ptr) {}
EXPORT void SDL_KeyDown(int keymod, int keysym) {}
EXPORT void SDL_KeyUp(int keymod, int keysym) {}

// function to run in a seperate thread for QT purposes
void launch_controllers(uint32_t ctrls) { 
    char argv_orig[] = "tasinput2";
    char *argv = argv_orig;
    int argc = 1;
    QApplication app(argc, &argv);

    // create controllers if needed
    for (uint32_t i = 0; i < 4; i++) {
        uint32_t cid = static_cast<uint32_t>(std::pow(2.0, static_cast<double>(i)));

        if (ctrls & cid) {
            controllers[i] = new Controller();
            controllers[i]->show();
        }
    }

     app.exec();
}

// initialize the controllers and open the QT windows
EXPORT void InitiateControllers(CONTROL_INFO controller_info) {
  lock.lock();
  const uint32_t CONTROLLER_COUNT = 1;  // TODO: not this

  qt_thread = std::optional(std::thread(launch_controllers, CONTROLLER_COUNT)); 
  lock.unlock();
}

// set the rom flag to open
EXPORT int RomOpen() {
  lock.lock();
  is_rom_open = true;
  lock.unlock();
}

// set the rom flag to closed and destroy the controllers
EXPORT int RomClosed() {
  lock.lock();
  is_rom_open = false;

  // destroy the application and merge the thread
  QCoreApplication::quit();
  qt_thread.value().join();

  for (int i = 0; i < 4; i++) {
    delete controllers[i];
  }

  lock.unlock();
}

// transmit the buttons being pressed to the core
EXPORT void GetKeys(int ctrl_number, BUTTONS* output) {
  lock.lock();
  try {
    *output = controllers[ctrl_number]->inputs->canonical_val(); 
  } catch (...) { /* just drop any exceptions */ }
  lock.unlock();
}
