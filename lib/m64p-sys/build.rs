/*
 * build.rs
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

extern crate bindgen;

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let header_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("headers");

    // download mupen64 headers
    let files = (
        header_dir.join("m64p_config.h"),
        header_dir.join("m64p_plugin.h"),
        header_dir.join("m64p_types.h")
    );
 
    // build with bindgen
    let bindings = bindgen::Builder::default()
        .header(files.0.to_str().unwrap())
        .header(files.1.to_str().unwrap())
        .header(files.2.to_str().unwrap())
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR")?).join("bindings.rs");
    bindings.write_to_file(out_path)?;

    Ok(())
}
