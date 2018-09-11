/*
Copyright (C) 2018 YangBy <yby@yangby.org>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate fibonacci_common;

use fibonacci_common::fibonacci;

fn main() {
    for argument in ::std::env::args().skip(1) {
        let x = usize::from_str_radix(&argument, 10).unwrap_or_else(|_| {
            panic!(format!(
                "Failed to parse input [{}] as a decimal number.",
                argument
            ))
        });
        let y = fibonacci(x);
        println!("fibonacci({}) = {}", x, y);
    }
}
