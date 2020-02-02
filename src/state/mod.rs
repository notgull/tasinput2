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

mod command;
mod error;

use crate::{Controller, CONTROLLER_COUNT};
use qt_widgets::{qt_core::QCoreApplication, qt_gui::QGuiApplication};
use std::{
    ffi::c_void,
    sync::{
        atomic::{AtomicBool, AtomicPtr},
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub use command::StateCommand;
pub use error::StateError;

// terminate the QT thread
fn terminate_qt() -> Result<(), StateError> {
    if unsafe { QCoreApplication::instance().is_null() } {
        Err(StateError::QtClosed)
    } else {
        unsafe {
            QCoreApplication::exit_0a();
        }
        Ok(())
    }
}

// manager for the inner thread
// TODO: remove unwraps
fn state_manager(tx: Sender<Result<(), StateError>>, rx: Receiver<StateCommand>) {
    let mut app_is_running = false;
    let continue_loop = Arc::new(Mutex::new(AtomicBool::new(true)));
    let controllers: [Controller; CONTROLLER_COUNT as usize] = Default::default();

    'stateloop: loop {
        match rx.recv().unwrap() {
            StateCommand::NoOp => tx.send(Ok(())).unwrap(),
            StateCommand::End => {
                *(continue_loop.lock().unwrap()).get_mut() = false;
                if let Err(e) = terminate_qt() {
                    tx.send(Err(e)).unwrap();
                } else {
                    tx.send(Ok(())).unwrap();
                }
            }
            StateCommand::StartQT => {
                if !(unsafe { QCoreApplication::instance().is_null() }) {
                    tx.send(Err(StateError::QtOpen)).unwrap();
                } else {
                    unsafe {
                        QGuiApplication::exec();
                    }
                    app_is_running = true;
                    tx.send(Ok(())).unwrap();
                }
            }
            StateCommand::EndQT => tx.send(terminate_qt()).unwrap(),
            StateCommand::InitializeController(control) => {}
            StateCommand::DeleteController(control) => {}
        }

        if *(continue_loop.lock().unwrap().get_mut()) {
            break 'stateloop;
        }
    }
}

/// Represents the current state of the program as a whole
pub struct Tasinput2State {
    pub context: Option<AtomicPtr<c_void>>,
    handle: Option<JoinHandle<()>>,
    tx: Sender<StateCommand>,
    rx: Receiver<Result<(), StateError>>,
}

impl Tasinput2State {
    /// Creates a new state.
    pub fn new() -> Tasinput2State {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let handle = Some(thread::spawn(move || {
            let tx = tx2;
            let rx = rx1;
            state_manager(tx, rx);
        }));

        Tasinput2State {
            context: None,
            handle,
            tx: tx1,
            rx: rx2,
        }
    }

    // send a command to the thread
    fn send_cmd(&self, command: StateCommand) -> Result<(), StateError> {
        self.tx.send(command)?;
        match self.rx.recv() {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(StateError::RecvError(e)),
        }
    }

    /// Start up QT
    pub fn start_qt(&self) -> Result<(), StateError> {
        self.send_cmd(StateCommand::StartQT)
    }

    /// End the QT context
    pub fn end_qt(&self) -> Result<(), StateError> {
        self.send_cmd(StateCommand::EndQT)
    }

    /// End the current loop
    pub fn end(&mut self) -> Result<(), StateError> {
        if let Err(e) = self.end_qt() {
            eprintln!("Unable to end QT: {:?}... Proceeding anyways", e);
        }

        self.context = None;
        self.send_cmd(StateCommand::End)?;
        match self.handle.take() {
            Some(handle) => match handle.join() {
                Ok(_) => {
                    self.handle = None;
                    Ok(())
                }
                Err(_) => Err(StateError::ThreadJoinPanic),
            },
            None => Err(StateError::ThreadHandleNonexistant),
        }
    }
}

impl Default for Tasinput2State {
    fn default() -> Tasinput2State {
        Self::new()
    }
}
