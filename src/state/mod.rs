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

use crate::{Controller, CONTROLLER_COUNT, Inputs};
use qt_widgets::{
    cpp_core::{MutPtr, MutRef},
    qt_core::{QCoreApplication, Slot},
    qt_gui::QGuiApplication,
};
use std::{
    ffi::{c_void, CString},
    sync::{
        atomic::{AtomicBool, AtomicPtr},
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub use command::{StateCommand, StateResponse};
pub use error::StateError;

// terminate the QT thread
fn terminate_qt() -> Result<StateResponse, StateError> {
    if unsafe { QCoreApplication::instance().is_null() } {
        Err(StateError::QtClosed)
    } else {
        unsafe {
            QCoreApplication::exit_0a();
        }
        Ok(StateResponse::None)
    }
}

// helper function to convert a vector of u8's into i8's
fn vec_u8_to_i8(vec: Vec<u8>) -> Vec<i8> {
    let mut v = std::mem::ManuallyDrop::new(vec);

    // the various parts of the vector: pointer, len, and capacity
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}

// start the QT thread
fn start_qt() -> Result<StateResponse, StateError> {
    if !(unsafe { QCoreApplication::instance().is_null() }) {
        Err(StateError::QtOpen)
    } else {
        // generate the argv
        let mut state_library = vec_u8_to_i8(CString::new("StaticLibrary")?.as_bytes().to_vec());
        let state_library_pointer = unsafe { MutPtr::from_raw(&mut state_library.as_mut_ptr()) };
        let mut one = 1;
        let reference_to_one = unsafe { MutRef::from_raw_ref(&mut one) };

        unsafe {
            let _app = QGuiApplication::new_2a(reference_to_one, state_library_pointer);
            thread::spawn(move || {
                QGuiApplication::exec();
            });
        }

        std::mem::forget(state_library);
        std::mem::forget(one);

        Ok(StateResponse::None)
    }
}

// manager for the inner thread
// TODO: remove unwraps
fn state_manager(tx: Sender<Result<StateResponse, StateError>>, rx: Receiver<StateCommand>) {
    let continue_loop = Arc::new(Mutex::new(AtomicBool::new(true)));
    let mut controllers: [Controller; CONTROLLER_COUNT as usize] = Default::default();

    'stateloop: loop {
        match rx.recv().unwrap() {
            StateCommand::NoOp => tx.send(Ok(StateResponse::None)).unwrap(),
            StateCommand::End => {
                *(continue_loop.lock().unwrap()).get_mut() = false;
                if let Err(e) = terminate_qt() {
                    tx.send(Err(e)).unwrap();
                } else {
                    tx.send(Ok(StateResponse::None)).unwrap();
                }
            }
            StateCommand::StartQT => tx.send(start_qt()).unwrap(),
            StateCommand::EndQT => tx.send(terminate_qt()).unwrap(),
            StateCommand::InitializeController(control) => {
                tx.send(match controllers[control].start_thread() {
                    Ok(_c) => Ok(StateResponse::None),
                    Err(e) => Err(StateError::ControllerError(e)),
                })
                .unwrap();
            }
            StateCommand::DeleteController(control) => {
                tx.send(match controllers[control].stop_thread() {
                    Ok(_c) => Ok(StateResponse::None),
                    Err(e) => Err(StateError::ControllerError(e)),
                })
                .unwrap();
            }
            StateCommand::GetInputs(control) => {
                tx.send(match controllers[control].get_inputs() {
                    Ok(i) => Ok(StateResponse::Inputs(i)),
                    Err(e) => Err(StateError::ControllerError(e)),
                }).unwrap();
            }
        }

        if *(continue_loop.lock().unwrap().get_mut()) {
            break 'stateloop;
        }
    }
}

/// Represents the current state of the program as a whole
pub struct Tasinput2State {
    pub romopen: AtomicBool,
    handle: Option<JoinHandle<()>>,
    tx: Sender<StateCommand>,
    rx: Receiver<Result<StateResponse, StateError>>,
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
            romopen: AtomicBool::new(false),
            handle,
            tx: tx1,
            rx: rx2,
        }
    }

    // send a command to the thread
    fn send_cmd(&self, command: StateCommand) -> Result<(), StateError> {
        self.tx.send(command)?;
        match self.rx.recv() {
            Ok(Ok(_)) => Ok(()),
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
            dprintln!("Unable to end QT: {:?}... Proceeding anyways", e);
        }

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

    /// Start up a certain controller
    pub fn start_controller(&self, control: usize) -> Result<(), StateError> {
        self.send_cmd(StateCommand::InitializeController(control))
    }

    /// Shut down a certain controller
    pub fn stop_controller(&self, control: usize) -> Result<(), StateError> {
        self.send_cmd(StateCommand::DeleteController(control))
    }

    /// Get inputs for a certain controller.
    pub fn get_inputs(&self, control: usize) -> Result<Inputs, StateError> {
        /*if !(self.is_active()) {
            return Err(StateError::StaticMsg("Already deactivated"));
        }*/

        self.tx.send(StateCommand::GetInputs(control))?;
        match self.rx.recv()? {
            Ok(StateResponse::Inputs(i)) => Ok(i),
            Ok(_) => Err(StateError::StaticMsg("Unexpected enum variant")),
            Err(e) => Err(e),
        }
    }
}

impl Default for Tasinput2State {
    fn default() -> Tasinput2State {
        Self::new()
    }
}
