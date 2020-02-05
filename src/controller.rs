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

type Checkbox = MutPtr<QCheckBox>;
type Spinbox = MutPtr<QSpinBox>;

/// Represents a window used to control inputs.
#[allow(dead_code)]
pub struct Controller<'a> {
    base_window: CppBox<QWidget>,
    inputs: &'a Arc<Mutex<Inputs>>,

    a: Checkbox,
    b: Checkbox,
    z: Checkbox,
    l: Checkbox,
    r: Checkbox,
    start: Checkbox,
    c_up: Checkbox,
    c_left: Checkbox,
    c_down: Checkbox,
    c_right: Checkbox,
    d_up: Checkbox,
    d_left: Checkbox,
    d_down: Checkbox,
    d_right: Checkbox,
    x: Spinbox,
    y: Spinbox,

    a_clicked: Slot<'a>,
    b_clicked: Slot<'a>,
    z_clicked: Slot<'a>,
    l_clicked: Slot<'a>,
    r_clicked: Slot<'a>,
    start_clicked: Slot<'a>,
    c_up_clicked: Slot<'a>,
    c_left_clicked: Slot<'a>,
    c_down_clicked: Slot<'a>,
    c_right_clicked: Slot<'a>,
    d_up_clicked: Slot<'a>,
    d_left_clicked: Slot<'a>,
    d_down_clicked: Slot<'a>,
    d_right_clicked: Slot<'a>,
    x_changed: Slot<'a>,
    y_changed: Slot<'a>,
}

impl<'a> Controller<'a> {
    /// Instantiate a new controller
    pub fn new(input_reference: &'a Arc<Mutex<Inputs>>) -> Controller<'a> {
        let mut base_window = unsafe { QWidget::new_0a() };
        unsafe { base_window.set_window_title(&QString::from_std_str("TAS Input")) };
        let mut layout = unsafe { QVBoxLayout::new_1a(&mut base_window).into_ptr() };

        // macro for creating a checkbox
        macro_rules! checkbox {
            ($name: expr) => {
                unsafe {
                    let mut cbox = QCheckBox::from_q_string(&QString::from_std_str($name));
                    layout.add_widget(&mut cbox);
                    cbox.into_ptr()
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

        // macro for creating a spin box
        macro_rules! spinbox {
            ($name: expr) => {
                unsafe {
                    /*let mut container = QWidget::new_0a();
                    let mut container_layout = QHBoxLayout::new_0a();

                    // label and spin box
                    let mut spin_label = QLabel::from_q_string(&QString::from_std_str($name));
                    let mut spin_box = QSpinBox::new_0a();
                    spin_box.set_minimum(-127);
                    spin_box.set_maximum(127);

                    container_layout.add_widget(&mut spin_box);
                    container_layout.add_widget(&mut spin_label);

                    container.set_layout(container_layout.into_ptr());

                    layout.add_widget(&mut container);
                    spin_box.into_ptr()*/

                    let mut spin_box = QSpinBox::new_0a();
                    layout.add_widget(&mut spin_box);
                    spin_box.into_ptr()
                }
            };
        }

        let x = spinbox!("X");
        let y = spinbox!("Y");

        // macro for creating a slot that corresponds to a checkbox
        macro_rules! clicked_handler {
            ($cbox: ident) => {
                unsafe {
                    Slot::new(move || {
                        (*input_reference.lock().unwrap()).$cbox = $cbox.is_checked();
                    })
                }
            };
            ($cbox: ident, $dname: ident.$pname: ident) => {
                unsafe {
                    Slot::new(move || {
                        (*input_reference.lock().unwrap()).$dname.$pname = $cbox.is_checked();
                    })
                }
            };
        };

        let a_clicked = clicked_handler!(a);
        let b_clicked = clicked_handler!(b);
        let z_clicked = clicked_handler!(z);
        let l_clicked = clicked_handler!(l);
        let r_clicked = clicked_handler!(r);
        let start_clicked = clicked_handler!(start);
        let c_up_clicked = clicked_handler!(c_up, c.up);
        let c_left_clicked = clicked_handler!(c_left, c.left);
        let c_down_clicked = clicked_handler!(c_down, c.down);
        let c_right_clicked = clicked_handler!(c_right, c.right);
        let d_up_clicked = clicked_handler!(d_up, d.up);
        let d_left_clicked = clicked_handler!(d_left, d.left);
        let d_down_clicked = clicked_handler!(d_down, d.down);
        let d_right_clicked = clicked_handler!(d_right, d.right);

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

            a,
            b,
            z,
            l,
            r,
            start,
            c_up,
            c_left,
            c_down,
            c_right,
            d_up,
            d_left,
            d_down,
            d_right,
            x,
            y,

            a_clicked,
            b_clicked,
            z_clicked,
            l_clicked,
            r_clicked,
            start_clicked,
            c_up_clicked,
            c_left_clicked,
            c_down_clicked,
            c_right_clicked,
            d_up_clicked,
            d_down_clicked,
            d_left_clicked,
            d_right_clicked,
            x_changed,
            y_changed,
        };

        unsafe {
            a.clicked().connect(&controller.a_clicked);
            b.clicked().connect(&controller.b_clicked);
            z.clicked().connect(&controller.z_clicked);
            l.clicked().connect(&controller.l_clicked);
            r.clicked().connect(&controller.r_clicked);
            start.clicked().connect(&controller.start_clicked);
            c_up.clicked().connect(&controller.c_up_clicked);
            c_left.clicked().connect(&controller.c_left_clicked);
            c_down.clicked().connect(&controller.c_down_clicked);
            c_right.clicked().connect(&controller.c_right_clicked);
            d_up.clicked().connect(&controller.d_up_clicked);
            d_left.clicked().connect(&controller.d_left_clicked);
            d_down.clicked().connect(&controller.d_down_clicked);
            d_right.clicked().connect(&controller.d_right_clicked);
            dprintln!("Spin boxes");
            x.value_changed().connect(&controller.x_changed);
            y.value_changed().connect(&controller.y_changed);
            dprintln!("Spin boxes finished");
        };

        dprintln!("Sockets connected");

        controller
    }
}
