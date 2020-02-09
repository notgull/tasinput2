/*
 * src/controller/mod.rs
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

#[macro_use]
mod macros;
mod button_panel;
mod joystick_panel;

use crate::Inputs;
use button_panel::ButtonPanel;
use joystick_panel::JoystickPanel;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    qt_core::{QString, Slot},
    QCheckBox, QHBoxLayout, QLabel, QSpinBox, QVBoxLayout, QWidget,
};
use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

pub type Checkbox = MutPtr<QCheckBox>;
pub type Spinbox = MutPtr<QSpinBox>;

/// Represents a window used to control inputs.
#[allow(dead_code)]
pub struct Controller<'a> {
    base_window: CppBox<QWidget>,
    inputs: &'a Arc<Mutex<Inputs>>,

    buttons: ButtonPanel<'a>,
    joystick: JoystickPanel<'a>,
}

impl<'a> Controller<'a> {
    /// Instantiate a new controller
    pub fn new(input_reference: &'a Arc<Mutex<Inputs>>) -> Controller<'a> {
        let mut base_window = unsafe { QWidget::new_0a() };
        unsafe { base_window.set_window_title(&QString::from_std_str("TAS Input")) };
        let mut layout = unsafe { QVBoxLayout::new_1a(&mut base_window).into_ptr() };

        let mut joystick = JoystickPanel::new(input_reference);
        unsafe { layout.add_widget(joystick.container.as_mut_ptr()) };

        let mut buttons = ButtonPanel::new(input_reference);
        unsafe { layout.add_widget(buttons.container.as_mut_ptr()) };

        unsafe { base_window.show() };

        let controller = Controller {
            base_window,
            inputs: input_reference,

            buttons,
            joystick,
        };

        controller
    }
}
