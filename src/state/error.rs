/*
 * src/state/error.rs
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

use super::StateCommand;
use std::sync::{
    atomic::AtomicBool,
    mpsc::{RecvError, SendError},
    PoisonError,
};
use thiserror::Error;

/// An error that can occur in state operations.
#[derive(Debug, Error)]
pub enum StateError {
    #[error("{0}")]
    StaticMsg(&'static str),
    #[error("An error occurred while receiving information from a thread")]
    RecvError(#[from] RecvError),
    #[error("An error occurred while sending information to the parent thread")]
    SendErrorPT(#[from] SendError<StateCommand>),
    #[error("An error occurred while sending information to the child thread")]
    SendErrorCT(#[from] SendError<()>),
    #[error("Unable to access mutex containing atomic boolean")]
    Mutex(#[from] PoisonError<AtomicBool>),
    #[error("QT is already open")]
    QtOpen,
    #[error("QT is already closed")]
    QtClosed,
    #[error("Thread handle does not exist")]
    ThreadHandleNonexistant,
    #[error("Unable to join thread")]
    ThreadJoinPanic,
}
