/*
 * src/debug.rs
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

use crate::STATE;
use std::{
    ffi::{c_void, CString},
    fmt::{self, Write},
    os::raw::c_char,
    sync::{Arc, Mutex},
};

/// Object that sends debug messages.
pub struct Debugger {
    pub debug_fn: unsafe extern "C" fn(*mut c_void, m64p_sys::m64p_msg_level, *const c_char),
}

impl Write for Debugger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // convert string to a C string
        let input = CString::new(s).unwrap_or_else(|e| {
            let mut v = e.into_vec();
            v.retain(|x| *x != 0);

            #[allow(clippy::or_fun_call)]
            CString::new(v).unwrap_or(CString::new("Unable to process error").unwrap())
        });

        // if we can't get a lock on either mutex, print error to eprint instead
        let mut state_lock = match STATE.lock() {
            Ok(l) => l,
            Err(_) => {
                eprint!("State Unavailable: {}", s);
                return Ok(());
            }
        };

        let context = match state_lock.context {
            Some(ref mut l) => l.get_mut(),
            None => {
                eprint!("Context Unavailable: {}", s);
                return Ok(());
            }
        };

        unsafe {
            (self.debug_fn)(
                *context,
                m64p_sys::m64p_msg_level_M64MSG_ERROR,
                input.into_raw(),
            );
        }
        Ok(())
    }
}

// functions for debugging a debug callback
lazy_static! {
    pub static ref DEBUG_OUT: Arc<Mutex<Option<Debugger>>> = Arc::new(Mutex::new(None));
}

// internal use printing function
#[doc(hidden)]
pub fn _dprint(args: fmt::Arguments) {
    let mut dlock = match DEBUG_OUT.lock() {
        Ok(dl) => dl,
        Err(_) => {
            eprint!("{}", args);
            return;
        }
    };

    let lock = dlock.as_mut();

    match lock {
        Some(l) => l.write_fmt(args).unwrap(),
        None => eprint!("{}", args)
    };
}

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => ($crate::debug::_dprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dprintln {
    () => ($crate::dprint!("\n"));
    ($($arg:tt)*) => ($crate::dprint!("{}\n", format_args!($($arg)*)));
}
