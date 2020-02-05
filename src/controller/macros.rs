/*
 * src/controller/macros.rs
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

// checkbox macro
#[macro_export]
macro_rules! checkbox {
    ($name: expr, $layout: ident) => {
        unsafe {
            let mut cbox = QCheckBox::from_q_string(&QString::from_std_str($name));
            $layout.add_widget(&mut cbox);
            cbox.into_ptr()
        }
    };
}

// macro for creating a slot that corresponds to a checkbox
#[macro_export]
macro_rules! clicked_handler {
    ($inputs: ident, $cbox: ident) => {
        unsafe {
            Slot::new(move || {
                (*$inputs.lock().unwrap()).$cbox = $cbox.is_checked();
            })
        }
    };
    ($inputs: ident, $cbox: ident, $dname: ident.$pname: ident) => {
        unsafe {
            Slot::new(move || {
                (*$inputs.lock().unwrap()).$dname.$pname = $cbox.is_checked();
            })
        }
    };
}
