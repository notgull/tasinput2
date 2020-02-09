/*
 * src/state/qt_thread.rs
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

use crate::{Controller, Inputs, CONTROLLER_COUNT};
use qt_widgets::QApplication;
use std::sync::{Arc, Mutex};

pub unsafe fn qt_thread(
    controllers: [bool; CONTROLLER_COUNT],
    inputs: Arc<[Arc<Mutex<Inputs>>; CONTROLLER_COUNT]>,
) {
    QApplication::init(move |_| {
        let mut controller_windows = Vec::new();

        // start up the controller windows
        //let mut controller_windows = [None; CONTROLLER_COUNT];
        for (i, do_init) in controllers.iter().enumerate() {
            if !do_init {
                continue;
            }

            dprintln!("Creating controller #{}", i);

            controller_windows.push(Controller::new(&inputs[i]));
        }

        QApplication::exec()
    });
}
