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
use qt_widgets::QWidget;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

mod command;
mod error;
mod manager;
mod response;

pub use command::ControllerCommand;
pub use error::ControllerError;
pub use response::ControllerResponse;
use crate::state::StateError;

/// Represents a controller's dialog box used to control inputs.
pub struct Controller {
    tx: Sender<ControllerCommand>,
    rx: Receiver<ControllerResponse>,
    temp_tx: Option<Sender<ControllerResponse>>,
    temp_rx: Option<Receiver<ControllerCommand>>,
    handle: Option<JoinHandle<()>>,
}

impl Controller {
    /// Create a new instance of a controller
    pub fn new() -> Controller {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        Controller {
            tx: tx1,
            rx: rx2,
            temp_tx: Some(tx2),
            temp_rx: Some(rx1),
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
            // unwrap here is safe because is_active assures that handle is Some
            return Err(ControllerError::StaticMsg("Unable to join thread"));
        }

        let (new_tx1, new_rx1) = mpsc::channel();
        let (new_tx2, new_rx2) = mpsc::channel();
        self.tx = new_tx1;
        self.rx = new_rx2;
        self.temp_tx = Some(new_tx2);
        self.temp_rx = Some(new_rx1);
        self.handle = None;
        Ok(true)
    }

    /// Start a controller thread.
    pub fn start_thread(&mut self) -> Result<(), ControllerError> {
        // if we are currently active, stop the thread
        if self.is_active() {
            match self.stop_thread() {
                Err(e) => return Err(e),
                Ok(true) => { /* no-op */ }
                Ok(false) => return Err(ControllerError::StaticMsg("Unable to stop thread")),
            };
        }

        // check to make sure that we have temporary mpsc's set up already
        if self.temp_tx.is_none() || self.temp_rx.is_none() {
            return Err(ControllerError::StaticMsg(
                "Temporary channels are not in place",
            ));
        }

        // start the thread
        let thread_tx = self.temp_tx.take().unwrap();
        let thread_rx = self.temp_rx.take().unwrap();
        self.handle = Some(thread::spawn(move || {
            let tx = thread_tx;
            let rx = thread_rx;
            manager::controller_manager(tx, rx);
        }));
        self.temp_tx = None;
        self.temp_rx = None;

        Ok(())
    }

    /// Get the inputs
    pub fn get_inputs(&self) -> Result<Inputs, ControllerError> {
        if !(self.is_active()) {
            return Err(ControllerError::ControllerNotActive);
        }

        self.tx.send(ControllerCommand::GetInputs)?;
        match self.rx.recv()? {
            ControllerResponse::Inputs(i) => Ok(i),
            _ => Err(ControllerError::EnumMismatch)
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
