/*
 * src/controller/error.rs
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

use super::ControllerCommand;
use std::sync::mpsc::{RecvError, SendError};
use thiserror::Error;

/// An error that could occur from interfacing with the controller.
#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("Unable to send data to thread")]
    Send(#[from] SendError<ControllerCommand>),
    #[error("Unable to receive data from master thread")]
    Recv(#[from] RecvError),
    #[error("An unspecified error occurred: {0}")]
    StaticMsg(&'static str),
    #[error("Controller is not active")]
    ControllerNotActive,
    #[error("Unexpected enum mismatch")]
    EnumMismatch,
}
