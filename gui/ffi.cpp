/*
 * gui/ffi.cpp
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

#include "about.h"

extern "C" {
  void show_about_box(void *parent_ptr) {
    ShowAboutDialog(parent_ptr);
  }
}
