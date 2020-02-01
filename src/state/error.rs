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

use std::sync::{
  atomic::AtomicBool,
  mpsc::{RecvError, SendError},
  PoisonError,
};
use super::StateCommand;
use thiserror::Error;

/// An error that can occur in state operations.
#[define(Debug, Error)]
pub enum StateError {
  #[error("{0}")]
  StaticMsg(&'static str),
  #[error("An error occurred while sending information to a thread")]
  SendError(#[from] SendError),
  #[error("An error occurred while receiving information from the parent thread")]
  RecvErrorPT(#[from] RecvError<StateCommand>),
  #[error("An error occurred while receiving information from the child thread")]
  RecvErrorCT(#[from] RecvError<()>),
  #[error("Unable to access mutex containing atomic boolean")] 
  Mutex(#[from] PoisonError<AtomicBool>),
  #[error("QT is already open")]
  QtOpen,
  #[error("QT is already closed")]
  QtClosed,
}
