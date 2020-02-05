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

use button_panel::ButtonPanel;
use crate::Inputs;
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
    x: Spinbox,
    y: Spinbox,
    x_changed: Slot<'a>,
    y_changed: Slot<'a>,
}

impl<'a> Controller<'a> {
    /// Instantiate a new controller
    pub fn new(input_reference: &'a Arc<Mutex<Inputs>>) -> Controller<'a> {
        let mut base_window = unsafe { QWidget::new_0a() };
        unsafe { base_window.set_window_title(&QString::from_std_str("TAS Input")) };
        let mut layout = unsafe { QVBoxLayout::new_1a(&mut base_window).into_ptr() };

        let mut buttons = ButtonPanel::new(input_reference);
        unsafe { layout.add_widget(buttons.container.as_mut_ptr()) };

        // macro for creating a spin box
        macro_rules! spinbox {
            ($name: expr) => {
                unsafe {
                    /*let mut container = QWidget::new_0a();
                    let mut container_layout = QHBoxLayout::new_0a();

                    // label and spin box
                    let mut spin_label = QLabel::from_q_string(&QString::from_std_str($name));
                    let mut spin_box = QSpinBox::new_0a();

                    container_layout.add_widget(&mut spin_box);
                    container_layout.add_widget(&mut spin_label);

                    container.set_layout(container_layout.into_ptr());

                    layout.add_widget(&mut container);
                    spin_box.into_ptr()*/

                    let mut spin_box = QSpinBox::new_0a();
                    spin_box.set_minimum(-127);
                    spin_box.set_maximum(127);
                    layout.add_widget(&mut spin_box);
                    spin_box.into_ptr()
                }
            };
        }

        let x = spinbox!("X");
        let y = spinbox!("Y");

        // macro for creating a slot that corresponds to a spinbox
        macro_rules! changed_handler {
            ($field_name: ident) => {
                unsafe {
                    Slot::new(move || {
                        (*input_reference.lock().unwrap()).$field_name =
                            $field_name.value().try_into().unwrap();
                    })
                }
            };
        };

        let x_changed = changed_handler!(x);
        let y_changed = changed_handler!(y);

        unsafe { base_window.show() };

        let controller = Controller {
            base_window,
            inputs: input_reference,

            buttons,

            x,
            y,

            x_changed,
            y_changed,
        };

        unsafe {
            x.value_changed().connect(&controller.x_changed);
            y.value_changed().connect(&controller.y_changed);
        };

        controller
    }
}
