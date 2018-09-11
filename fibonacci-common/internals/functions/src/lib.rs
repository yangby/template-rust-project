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

extern crate num_bigint;

pub use num_bigint::BigUint;

pub fn fibonacci(x: usize) -> BigUint {
    if x < 2 {
        BigUint::from(x)
    } else {
        let mut y = (BigUint::from(0usize), BigUint::from(1usize));
        let mut z = x;
        while z > 1 {
            y.0 += &y.1;
            y = (y.1, y.0);
            z -= 1;
        }
        y.1
    }
}
