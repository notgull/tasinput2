/*
 * src/controller/button_panel/directional.rs
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
    QWidget, QCheckBox, QHBoxLayout, QVBoxLayout,
};
use std::sync::{Arc, Mutex};
use super::Checkbox;

/// Which directional does this widget correspond to?
pub enum DirectionalType {
    c,
    d
}

/// A checkbox series corresponding to one of the directional inputs.
#[allow(dead_code)]
pub struct DirectionalWidget<'a> {
    pub container: CppBox<QWidget>,
    input_reference: &'a Arc<Mutex<Inputs>>,

    up: Checkbox,
    down: Checkbox,
    left: Checkbox,
    right: Checkbox,

    h_container: MutPtr<QWidget>,

    up_clicked: Slot<'a>,
    down_clicked: Slot<'a>,
    left_clicked: Slot<'a>,
    right_clicked: Slot<'a>,
}

impl<'a> DirectionalWidget<'a> {
    /// Instantiate a new directional widget.
    pub fn new(input_reference: &'a Arc<Mutex<Inputs>>, dtype: DirectionalType) -> DirectionalWidget<'a> {
        let mut container = unsafe { QWidget::new_0a() };
        let mut layout = unsafe { QVBoxLayout::new_1a(&mut container).into_ptr() };

        let letter_representing = match &dtype {
            c => "C",
            d => "D"
        };

        let up = checkbox!(format!("{} Up", letter_representing), layout);

        let mut horizontal_container = unsafe { QWidget::new_0a() };
        let mut h_layout = unsafe { QHBoxLayout::new_1a(horizontal_container.as_mut_ptr()).into_ptr() };

        let left = checkbox!(format!("{} Left", letter_representing), h_layout);
        let right = checkbox!(format!("{} Right", letter_representing), h_layout);

        unsafe { layout.add_widget(horizontal_container.as_mut_ptr()); };
        let mut horizontal_container = unsafe { horizontal_container.into_ptr() };

        let down = checkbox!(format!("{} Down", letter_representing), layout);

        // create handlers
        let (up_clicked, down_clicked, left_clicked, right_clicked) = match dtype {
            c => (
                    clicked_handler!(input_reference, up, c.up),
                    clicked_handler!(input_reference, down, c.down),
                    clicked_handler!(input_reference, left, c.left),
                    clicked_handler!(input_reference, right, c.right)
            ),
            d => (
                clicked_handler!(input_reference, up, d.up),
                clicked_handler!(input_reference, down, d.down),
                clicked_handler!(input_reference, left, d.left),
                clicked_handler!(input_reference, right, d.right)
            ),
        };

        let dw = DirectionalWidget {
            container,
            h_container: horizontal_container,
            input_reference,

            up, down, left, right,
            up_clicked, down_clicked, left_clicked, right_clicked,
        };

        unsafe {
            up.clicked().connect(&dw.up_clicked);
            down.clicked().connect(&dw.down_clicked);
            left.clicked().connect(&dw.left_clicked);
            right.clicked().connect(&dw.right_clicked);
        };

        dw
    }
}