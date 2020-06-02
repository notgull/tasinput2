/*
 * include/plugin.h 
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

// c bindings to the controllers with QT

#ifndef PLUGIN_H
#define PLUGIN_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include "m64p/m64p_plugin.h"

void lock_mutex();
void unlock_mutex();
void launch_controllers(uint32_t ctrls);
void deinit_controllers();
BUTTONS get_ctrl_keys(int ctrl_number);

#ifdef __cplusplus
}
#endif

#endif
