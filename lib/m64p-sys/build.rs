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
extern crate futures;
extern crate reqwest;
extern crate tempfile;
extern crate tokio;

use std::io::Write;
use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

// download file helper
async fn download_file<'a>(
    client: &'a reqwest::Client,
    dir_path: &'a Path,
    url: &'static str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;

    // get the out path
    let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.h");
    let fname = dir_path.join(fname);
    let mut dest = File::create(&fname)?;

    dest.write_all(&response.bytes().await?)?;
    Ok(fname)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let tmp_dir = tempfile::Builder::new().prefix("m64p_headers").tempdir()?;

    // download mupen64 headers
    let files = futures::join!(
        download_file(&client, tmp_dir.path(), "https://raw.githubusercontent.com/mupen64plus/mupen64plus-core/master/src/api/m64p_plugin.h"),
        download_file(&client, tmp_dir.path(), "https://raw.githubusercontent.com/mupen64plus/mupen64plus-core/master/src/api/m64p_config.h"),
        download_file(&client, tmp_dir.path(), "https://raw.githubusercontent.com/mupen64plus/mupen64plus-core/master/src/api/m64p_types.h")
    );

    // build with bindgen
    let bindings = bindgen::Builder::default()
        .header(files.0.unwrap().to_str().unwrap())
        .header(files.1.unwrap().to_str().unwrap())
        .header(files.2.unwrap().to_str().unwrap())
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR")?).join("bindings.rs");
    bindings.write_to_file(out_path)?;

    Ok(())
}
