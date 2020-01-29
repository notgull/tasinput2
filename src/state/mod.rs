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

use crate::{Controller, CONTROLLER_COUNT};
use command::StateCommand;
use qt_widgets::{qt_core::QCoreApplication, qt_gui::QGuiApplication};
use std::{
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

// terminate the QT thread
fn terminate_qt() -> Result<(), &'static str> {
    if unsafe { QCoreApplication::instance().is_null() } {
        Err("Application is null")
    } else {
        unsafe {
            QCoreApplication::exit_0a();
        }
        Ok(())
    }
}

// manager for the inner thread
// TODO: remove unwraps
fn state_manager(tx: Sender<Result<(), &'static str>>, rx: Receiver<StateCommand>) {
    let mut app_is_running = false;
    let continue_loop = Arc::new(Mutex::new(AtomicBool::new(true)));

    'stateloop: loop {
        match rx.recv().unwrap() {
            StateCommand::NoOp => tx.send(Ok(())).unwrap(),
            StateCommand::End => {
                *(continue_loop.lock().unwrap()).get_mut() = false;
                if let Ok(()) = terminate_qt() { };
                tx.send(Ok(())).unwrap();
            }
            StateCommand::StartQT => {
                if !(unsafe { QCoreApplication::instance().is_null() }) {
                    tx.send(Err("QGuiApplication is already open")).unwrap();
                } else {
                    unsafe {
                        QGuiApplication::exec();
                    }
                    app_is_running = true;
                    tx.send(Ok(())).unwrap();
                }
            }
            StateCommand::EndQT => tx.send(terminate_qt()).unwrap(),
        }

        if *(continue_loop.lock().unwrap().get_mut()) {
            break 'stateloop;
        }
    }
}

/// Represents the current state of the program as a whole
pub struct Tasinput2State {
    handle: JoinHandle<()>,
    tx: Sender<StateCommand>,
    rx: Receiver<Result<(), &'static str>>,
}

impl Tasinput2State {
    /// Creates a new state.
    pub fn new() -> Tasinput2State {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();

        let handle = thread::spawn(move || {
            let tx = tx2;
            let rx = rx1;
            state_manager(tx, rx);
        });

        Tasinput2State {
            handle,
            tx: tx1,
            rx: rx2,
        }
    }
}

impl Default for Tasinput2State {
    fn default() -> Tasinput2State { Self::new() }
}