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

use qt_widgets::QWidget;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

mod command;
mod error;

pub use command::ControllerCommand;
pub use error::ControllerError;

/// Represents a controller's dialog box used to control inputs.
pub struct Controller {
    tx: Sender<ControllerCommand>,
    rx: Receiver<ControllerCommand>,
    temp_tx: Option<Sender<ControllerCommand>>,
    temp_rx: Option<Receiver<ControllerCommand>>,
    handle: Option<JoinHandle<()>>,
}

impl Controller {
    /// Create a new instance of a controller
    pub fn new() -> Controller {
        let (tx, rx) = mpsc::channel();
        let (temp_tx, temp_rx) = mpsc::channel();

        Controller {
            tx,
            rx,
            temp_tx: Some(temp_tx),
            temp_rx: Some(temp_rx),
            handle: None,
        }
    }

    /// Tell whether this controller is active.
    pub fn is_active(&self) -> bool {
        self.handle.is_some()
    }

    /// Stop the current controller thread.
    pub fn stop_thread(&mut self) -> Result<bool, ControllerError> {
        // if we aren't active at the moment we can bail early
        if !(self.is_active()) {
            return Ok(false);
        }

        self.tx.send(ControllerCommand::End)?;
        if self.handle.take().unwrap().join().is_err() {
            return Err(ControllerError::StaticMsg("Unable to join thread"));
        }

        let (new_tx, new_rx) = mpsc::channel();
        self.temp_tx = Some(new_tx);
        self.temp_rx = Some(new_rx);
        self.handle = None;
        Ok(true)
    }
}

pub struct ControllerData {
    window: QWidget,
    controller_id: u8,
}
