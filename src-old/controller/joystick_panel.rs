/*
 * src/controller/joystick_panel.rs
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

use super::Spinbox;
use crate::Inputs;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    q_frame::Shape,
    qt_core::{QString, Slot, SlotOfInt},
    qt_gui::{QBrush, QColor, QPainter, QPen},
    QFrame, QHBoxLayout, QLabel, QSpinBox, QVBoxLayout, QWidget,
};
use std::{
    cell::RefCell,
    convert::TryInto,
    rc::Rc,
    sync::{Arc, Mutex},
};

// helper function to draw the QPicture containing the targets

/// The panel allowing for manipulation of the joystick.
#[allow(dead_code)]
pub struct JoystickPanel<'a> {
    pub container: CppBox<QWidget>,
    input_reference: &'a Arc<Mutex<Inputs>>,

    joystick_canvas: MutPtr<QFrame>,

    x: Spinbox,
    y: Spinbox,

    spinbox_container: MutPtr<QWidget>,
    x_container: MutPtr<QWidget>,
    y_container: MutPtr<QWidget>,

    x_changed: SlotOfInt<'a>,
    y_changed: SlotOfInt<'a>,

    //    canvas_clicked: Slot<'a>,
    //    canvas_dragged: Slot<'a>,
    //    canvas_update: Slot<'a>,
    x_value: Rc<RefCell<i8>>,
    y_value: Rc<RefCell<i8>>,
}

impl<'a> JoystickPanel<'a> {
    /// Instantiate a new joystick panel.
    pub fn new(input_reference: &'a Arc<Mutex<Inputs>>) -> JoystickPanel<'a> {
        let mut container = unsafe { QWidget::new_0a() };
        let mut layout = unsafe { QHBoxLayout::new_1a(&mut container).into_ptr() };

        // create the canvas
        let mut joystick_canvas = unsafe { QFrame::new_0a() };
        unsafe {
            joystick_canvas.set_minimum_size_2a(200, 200);
            joystick_canvas.set_frame_shape(Shape::Box);
            joystick_canvas.set_line_width(2);
            joystick_canvas.set_fixed_width(200);
            layout.add_widget(&mut joystick_canvas);
        };
        let mut joystick_canvas = unsafe { joystick_canvas.into_ptr() };

        // create the spinboxes
        let mut spinbox_container = unsafe { QWidget::new_0a() };
        let mut spinbox_layout = unsafe { QVBoxLayout::new_1a(&mut spinbox_container).into_ptr() };
        let (mut x_container, mut x) = spinbox!("X", spinbox_layout);
        let (mut y_container, mut y) = spinbox!("Y", spinbox_layout);
        unsafe { layout.add_widget(&mut spinbox_container) };
        let spinbox_container = unsafe { spinbox_container.into_ptr() };

        // x and y basic values
        let x_value = Rc::new(RefCell::new(0));
        let y_value = Rc::new(RefCell::new(0));

        let x_ref = x_value.clone();
        let y_ref = y_value.clone();

        // updater function
        let mut update_x = move |xval: i8| {
            let mut input = input_reference.lock().unwrap();
            input.x = xval;

            unsafe {
                x.set_value(xval.into());
            };

            *x_ref.borrow_mut() = xval;
        };

        let mut update_y = move |yval: i8| {
            let mut input = input_reference.lock().unwrap();
            input.y = yval;

            unsafe {
                y.set_value(yval.into());
            };

            *y_ref.borrow_mut() = yval;
        };

        let x_ref = x_value.clone();
        let y_ref = y_value.clone();

        // paint update function for canvas
        /*let canvas_update = unsafe {
            Slot::new(move || {
                let mut painter = QPainter::new_1a(joystick_canvas);

                let blue = QColor::from_rgb_3a(156, 179, 255);
                let mut blue_pen = QPen::from_q_color(&blue);
                blue_pen.set_width(6);

                painter.set_pen_q_pen(&blue_pen);
                painter.draw_ellipse_4_int(5, 5, 195, 195);
            })
        };*/

        let x_changed = unsafe {
            SlotOfInt::new(move |val| {
                update_x(val.try_into().unwrap());
            })
        };

        let y_changed = unsafe {
            SlotOfInt::new(move |val| {
                update_y(val.try_into().unwrap());
            })
        };

        let mut jp = JoystickPanel {
            container,
            input_reference,

            joystick_canvas,

            spinbox_container,
            x,
            y,
            x_container,
            y_container,

            x_value,
            y_value,

            x_changed,
            y_changed,
            //canvas_update,
        };

        unsafe {
            x.value_changed().connect(&jp.x_changed);
            y.value_changed().connect(&jp.y_changed);
            //joystick_canvas.slot_update().connect(&jp.canvas_update);
        };

        jp
    }
}
