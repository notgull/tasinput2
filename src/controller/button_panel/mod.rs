/*
 * src/controller/button_panel/mod.rs
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

mod directional;

pub use super::Checkbox;

use crate::Inputs;
use directional::{DirectionalWidget, DirectionalType};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    qt_core::{QString, Slot},
    QWidget, QCheckBox, QHBoxLayout, QVBoxLayout
};
use std::sync::{Arc, Mutex};

/// The panel used for buttons.
pub struct ButtonPanel<'a> {
    pub container: CppBox<QWidget>,
    inputs: &'a Arc<Mutex<Inputs>>,

    a: Checkbox,
    b: Checkbox,
    z: Checkbox,
    l: Checkbox,
    r: Checkbox,
    start: Checkbox,

    c: DirectionalWidget<'a>,
    d: DirectionalWidget<'a>,

    m_box: MutPtr<QWidget>,

    a_clicked: Slot<'a>,
    b_clicked: Slot<'a>,
    z_clicked: Slot<'a>,
    l_clicked: Slot<'a>,
    r_clicked: Slot<'a>,
    start_clicked: Slot<'a>,
}

impl<'a> ButtonPanel<'a> {
    /// Instantiate a new button panel.
    pub fn new(input_reference: &'a Arc<Mutex<Inputs>>) -> ButtonPanel<'a> {
        let mut container = unsafe { QWidget::new_0a() };
        let mut layout = unsafe { QHBoxLayout::new_1a(&mut container).into_ptr() };

        // create the D directional and add it
        let mut d = DirectionalWidget::new(input_reference, DirectionalType::d);
        unsafe { layout.add_widget(d.container.as_mut_ptr()) };

        // add just the l button to the layout
        let l = checkbox!("L", layout);

        // create a box with the Z, a, b, and start buttons in it
        let mut m_box = unsafe { QWidget::new_0a() };
        let mut m_layout = unsafe { QVBoxLayout::new_1a(&mut m_box).into_ptr() };

        let z = checkbox!("Z", m_layout);
        let a = checkbox!("A", m_layout);
        let b = checkbox!("B", m_layout);
        let start = checkbox!("Start", m_layout);

        unsafe { layout.add_widget(m_box.as_mut_ptr()) };

        // add just the r button to the layout
        let r = checkbox!("R", layout);

        // create the C directional and add it
        let mut c = DirectionalWidget::new(input_reference, DirectionalType::c);
        unsafe { layout.add_widget(c.container.as_mut_ptr()) };

        // create buttons clicks
        let a_clicked = clicked_handler!(input_reference, a);
        let b_clicked = clicked_handler!(input_reference, b);
        let z_clicked = clicked_handler!(input_reference, z);
        let l_clicked = clicked_handler!(input_reference, l);
        let r_clicked = clicked_handler!(input_reference, r);
        let start_clicked = clicked_handler!(input_reference, start);

        let buttons = ButtonPanel {
            container,
            inputs: input_reference,

            a, b, z, l, r, start, c, d, m_box: unsafe { m_box.into_ptr() },

            a_clicked,
            b_clicked,
            z_clicked,
            l_clicked,
            r_clicked,
            start_clicked
        };

        unsafe {
            a.clicked().connect(&buttons.a_clicked);
            b.clicked().connect(&buttons.b_clicked);
            z.clicked().connect(&buttons.z_clicked);
            l.clicked().connect(&buttons.l_clicked);
            r.clicked().connect(&buttons.r_clicked);
            start.clicked().connect(&buttons.start_clicked);
        };

        buttons
    }
}