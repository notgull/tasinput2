/*
 * src/plugin.cpp
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

#include "m64p/m64p_plugin.h"

#include "controller.hpp"

#include <cmath>
#include <mutex>
#include <optional>
#include <thread>

#include <QApplication>
#include <QCoreApplication>

std::optional<std::thread> qt_thread;
std::mutex lock;

Controller *controllers[4] = {nullptr, nullptr, nullptr, nullptr};

// lock/unlock the mutex
void lock_mutex() {
    lock.lock();
}

void unlock_mutex() {
    lock.unlock();
}

// function to run in a seperate thread for QT purposes
void _launch_controllers(uint32_t ctrls) {
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

// initialize the controllers
void launch_controllers(uint32_t ctrls) {
  qt_thread = std::optional(std::thread(_launch_controllers, ctrls));
}

// delete the controllers
void deinit_controllers() {
  QCoreApplication::quit();
  qt_thread.value().join();

  for (int i = 0; i < 4; i++) {
    if (controllers[i]) {
      delete controllers[i];
    }
  }
}

// get the keys for a controller
BUTTONS get_ctrl_keys(int ctrl_number) {
  return controllers[ctrl_number]->inputs->canonical_val();
}
