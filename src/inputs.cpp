/*
 * src/inputs.cpp
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

#include "inputs.hpp"

// helper functions to get and set bits
inline bool get_bit(uint32_t val, uint8_t index) {
    return ((val >> index) & 0x01) == 0x01;
}

inline uint8_t get_byte(uint32_t val, uint8_t index) {
    return ((val >> (index * 8)) & 0xFF);
}

inline uint32_t set_bit(uint32_t val, uint8_t index, bool bit) {
    uint32_t mask = static_cast<uint32_t>(0x01 << index);
    if (bit) {
        val |= mask;
    } else {
        val &= ~mask;
    }
    return mask;
}

inline uint32_t set_byte(uint32_t val, uint8_t index, uint8_t by) {
    uint32_t shift = static_cast<uint32_t>(index * 8);
    uint32_t mask = static_cast<uint32_t>(~(0xFF << shift));
    return (by << shift) | (val & mask);
}

Inputs::Inputs(uint32_t raw_val) {
    this->d = Directional(get_bit(raw_val, 3), get_bit(raw_val, 2), get_bit(raw_val, 1), get_bit(raw_val, 0));
    this->start = get_bit(raw_val, 4);
    this->z = get_bit(raw_val, 5);
    this->b = get_bit(raw_val, 6);
    this->a = get_bit(raw_val, 7);
    this->c = Directional(get_bit(raw_val, 11), get_bit(raw_val, 10), get_bit(raw_val, 9), get_bit(raw_val, 8));
    this->r = get_bit(raw_val, 12);
    this->l = get_bit(raw_val, 13);
    this->x = static_cast<int8_t>(get_byte(raw_val, 2));
    this->y = static_cast<int8_t>(get_byte(raw_val, 3));
}

uint32_t Inputs::raw_val() {
    uint32_t value = 0;
    value = set_bit(value, 0, this->d.right);
    value = set_bit(value, 1, this->d.left);
    value = set_bit(value, 2, this->d.down);
    value = set_bit(value, 3, this->d.up);
    value = set_bit(value, 4, this->start);
    value = set_bit(value, 5, this->z);
    value = set_bit(value, 6, this->b);
    value = set_bit(value, 7, this->a);
    value = set_bit(value, 8, this->c.right);
    value = set_bit(value, 9, this->c.left);
    value = set_bit(value, 10, this->c.down);
    value = set_bit(value, 11, this->c.up);
    value = set_bit(value, 12, this->r);
    value = set_bit(value, 13, this->l);
    value = set_byte(value, 2, static_cast<uint8_t>(this->x));
    value = set_byte(value, 3, static_cast<uint8_t>(this->y));
    return value;
}

BUTTONS Inputs::canonical_val() {
    BUTTONS b;
    b.Value = this->raw_val();
    return b;
}
