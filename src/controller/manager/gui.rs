/*
 * src/controller/manager/gui.rs
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

use crate::Inputs;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    qt_core::QString,
    QCheckBox, QSpinBox, QWidget,
};
use std::convert::TryInto;

/// Controller for the status GUI
pub struct ControllerGui {
    base_window: CppBox<QWidget>,
    a: CppBox<QCheckBox>,
    b: CppBox<QCheckBox>,
    z: CppBox<QCheckBox>,
    l: CppBox<QCheckBox>,
    r: CppBox<QCheckBox>,
    start: CppBox<QCheckBox>,
    c_up: CppBox<QCheckBox>,
    c_down: CppBox<QCheckBox>,
    c_left: CppBox<QCheckBox>,
    c_right: CppBox<QCheckBox>,
    d_up: CppBox<QCheckBox>,
    d_down: CppBox<QCheckBox>,
    d_left: CppBox<QCheckBox>,
    d_right: CppBox<QCheckBox>,
    x: CppBox<QSpinBox>,
    y: CppBox<QSpinBox>,
}

impl ControllerGui {
    /// Instantiate a new controller gui
    pub fn new() -> ControllerGui {
        let mut base_window = unsafe { QWidget::new_0a() };

        macro_rules! checkbox {
            ($ctxt: expr) => {
                unsafe {
                    QCheckBox::from_q_string_q_widget(
                        &QString::from_std_str($ctxt),
                        base_window.as_mut_ptr(),
                    )
                }
            };
        };

        let a = checkbox!("A");
        let b = checkbox!("B");
        let z = checkbox!("Z");
        let l = checkbox!("L");
        let r = checkbox!("R");
        let start = checkbox!("Start");
        let c_up = checkbox!("C Up");
        let c_left = checkbox!("C Left");
        let c_down = checkbox!("C Down");
        let c_right = checkbox!("C Right");
        let d_up = checkbox!("D Up");
        let d_left = checkbox!("D Left");
        let d_down = checkbox!("D Down");
        let d_right = checkbox!("D Right");

        macro_rules! spinbox {
            () => {
                unsafe {
                    let mut spin = QSpinBox::new_1a(base_window.as_mut_ptr());
                    spin.set_minimum(-127);
                    spin.set_maximum(127);
                    spin
                }
            };
        }

        let x = spinbox!();
        let y = spinbox!();

        ControllerGui {
            base_window,
            a,
            b,
            z,
            l,
            r,
            start,
            c_up,
            c_down,
            c_left,
            c_right,
            d_up,
            d_down,
            d_left,
            d_right,
            x,
            y,
        }
    }

    /// Show this controller GUI
    pub fn show(&mut self) {
        unsafe {
            self.base_window.show();
        }
    }

    /// Hide this controller GUI
    pub fn hide(&mut self) {
        unsafe {
            self.base_window.hide();
        }
    }

    /// Provide an input
    pub fn get_inputs(&self) -> Inputs {
        unsafe {
            Inputs::new(
                self.x.value().try_into().unwrap(),
                self.y.value().try_into().unwrap(),
                self.a.is_checked(),
                self.b.is_checked(),
                self.z.is_checked(),
                self.l.is_checked(),
                self.r.is_checked(),
                self.start.is_checked(),
                self.c_up.is_checked(),
                self.c_down.is_checked(),
                self.c_left.is_checked(),
                self.c_right.is_checked(),
                self.d_up.is_checked(),
                self.d_down.is_checked(),
                self.d_left.is_checked(),
                self.d_right.is_checked(),
            )
        }
    }
}
