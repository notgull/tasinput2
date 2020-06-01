/*
 * include/corelib.hpp
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

#ifndef TASINPUT2_CORELIB_HPP
#define TASINPUT2_CORELIB_HPP

#include <cstdlib>
#include "m64p/m64p_config.h"

// pointers to necessary functions
ptr_ConfigListSections _config_list_sections = nullptr;
ptr_ConfigOpenSection _config_open_section = nullptr;
ptr_ConfigDeleteSection _config_delete_section = nullptr;
ptr_ConfigListParameters _config_list_parameters = nullptr;
ptr_ConfigSetParameter _config_set_parameter = nullptr;

#endif
