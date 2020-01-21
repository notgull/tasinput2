/*
 * build.rs
 *
 * tasinput2 - Input plugin for generating tool assisted speedruns
 * Copyright (C) 2020 not_a_seagull
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

extern crate cc;

use std::process::Command;

fn main() {
    let files = &[
      "gui/about.cpp",
      "gui/ffi.cpp"
    ];

    let wx_flags_raw = Command::new("wx-config")
        .args(&["--cxxflags", "--libs"])
        .output()
        .unwrap()
        .stdout;
    let wx_flags = std::str::from_utf8(&wx_flags_raw).unwrap();

    let mut build = cc::Build::new();
    build.cpp(true);
    build.opt_level(2);
    build.flag("-fno-exceptions");
    build.flag("--std=c++11");

    for flag in wx_flags.replace("\n", " ").split_whitespace() {
        build.flag(flag);
    }

    for file in files {
        build.file(file);
    }

    build.compile("libguiadapter.a");
}