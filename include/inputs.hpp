/*
 * include/inputs.hpp
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

#ifndef INPUTS_HPP
#define INPUTS_HPP

#include <cstdint>
#include "m64p/m64p_plugin.h"

// Directional buttons.
class Directional {
 public:
  bool up;
  bool down;
  bool left;
  bool right;

  // Initialize a directional.
  constexpr Directional() : up(false), down(false), left(false), right(false) {}
  constexpr Directional(bool u, bool d, bool l, bool r)
      : up(u), down(d), left(l), right(r) {}
};

// Inputs that may or may not be activated on a controller.
class Inputs {
 public:
  bool a;
  bool b;
  bool z;
  bool l;
  bool r;
  Directional c;
  Directional d;
  bool start;
  int8_t x;
  int8_t y;

  // New input with default settings.
  constexpr Inputs()
      : a(false),
        b(false),
        z(false),
        l(false),
        r(false),
        start(false),
        x(0),
        y(0),
        c(),
        d() {}

  // inputs can be saved and loaded as u32's
  Inputs(uint32_t raw_val);
  uint32_t raw_val();

  // save and load inputs as the Button struct
  inline Inputs(BUTTONS buttons) : Inputs(buttons.Value){};
  BUTTONS canonical_val();
};

#endif
