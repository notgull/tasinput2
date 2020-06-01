/*
 * src/state/mod.rs
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

mod error;
mod qt_thread;

use crate::{Inputs, CONTROLLER_COUNT};
use qt_widgets::qt_core::QCoreApplication;
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

pub use error::StateError;

/// Represents the current state of the program as a whole
pub struct Tasinput2State {
    pub is_initialized: bool,
    pub is_rom_open: bool,
    pub is_gui_open: bool,
    inputs: Arc<[Arc<Mutex<Inputs>>; CONTROLLER_COUNT]>,
    qt_thread: Option<JoinHandle<()>>,
}

impl Tasinput2State {
    /// Creates a new state.
    pub fn new() -> Tasinput2State {
        Tasinput2State {
            is_initialized: false,
            is_rom_open: false,
            is_gui_open: false,
            inputs: Arc::new(array_init::array_init(|_| {
                Arc::new(Mutex::new(Inputs::from_value(0)))
            })),
            qt_thread: None,
        }
    }

    /// Initialize the QT thread
    pub fn start_qt(&mut self, controllers: u8) -> Result<(), StateError> {
        if !(unsafe { QCoreApplication::instance().is_null() }) || self.is_gui_open {
            return Err(StateError::QtOpen);
        }

        let controllers = [
            controllers & 1 != 0,
            controllers & 2 != 0,
            controllers & 4 != 0,
            controllers & 8 != 0,
        ];

        let inputs_cloned = self.inputs.clone();
        self.qt_thread = Some(thread::spawn(move || unsafe {
            qt_thread::qt_thread(controllers, inputs_cloned);
        }));

        self.is_gui_open = true;

        Ok(())
    }

    /// End the QT Thread
    pub fn end_qt(&mut self) -> Result<(), StateError> {
        if unsafe { QCoreApplication::instance().is_null() } {
            Err(StateError::QtClosed)
        } else {
            unsafe {
                QCoreApplication::quit();
            }

            match self.qt_thread.take().unwrap().join() {
                Ok(l) => l,
                Err(_) => return Err(StateError::ThreadJoinPanic),
            }

            self.qt_thread = None;
            self.is_gui_open = false;
            Ok(())
        }
    }

    /// Get the inputs for a specific controller.
    pub fn get_inputs(&self, control: usize) -> Inputs {
        *self.inputs[control].lock().unwrap()
    }
}

impl Default for Tasinput2State {
    fn default() -> Tasinput2State {
        Self::new()
    }
}
