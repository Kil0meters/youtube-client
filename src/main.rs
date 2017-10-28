//  Copyright (C) 2017  Kil0meters
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate gio;
extern crate glib;
extern crate gtk;
extern crate htmlescape;
extern crate pango;

extern crate regex;
extern crate reqwest;
extern crate select;

mod ui;
mod lib;

use std::process;

fn main() {
    match ui::run_app() {
        Ok(_) => {}
        Err(e) => {
            eprint!("Failed to run app: {}", e);
            process::exit(1);
        }
    }
}
