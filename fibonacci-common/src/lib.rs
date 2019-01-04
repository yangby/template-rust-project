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

extern crate fibonacci_functions;
extern crate fibonacci_macros;

pub use fibonacci_functions::{fibonacci as fibonacci_raw, BigUint};
use fibonacci_macros::construct_fibonacci_cache;

construct_fibonacci_cache!(FIBONACCI_CACHE, 512);
pub fn fibonacci(x: usize) -> BigUint {
    if x >= FIBONACCI_CACHE.len() {
        let l = FIBONACCI_CACHE.len();
        let mut y = (
            BigUint::from_bytes_be(&FIBONACCI_CACHE[l - 2][..]),
            BigUint::from_bytes_be(&FIBONACCI_CACHE[l - 1][..]),
        );
        let mut z = x - l + 1;
        while z > 0 {
            y.0 += &y.1;
            y = (y.1, y.0);
            z -= 1;
        }
        y.1
    } else {
        BigUint::from_bytes_be(&FIBONACCI_CACHE[x][..])
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn fibonacci_raw() {
        let y: Vec<usize> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (x, item) in y.iter().enumerate() {
            assert_eq!(super::fibonacci_raw(x), super::BigUint::from(*item));
        }
        assert_eq!(
            super::fibonacci_raw(64),
            super::BigUint::from(10_610_209_857_723usize)
        );
        assert_eq!(
            super::fibonacci_raw(93),
            super::BigUint::from(12_200_160_415_121_876_738usize)
        );
    }

    #[test]
    fn fibonacci() {
        let y: Vec<usize> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (x, item) in y.iter().enumerate() {
            assert_eq!(super::fibonacci(x), super::BigUint::from(*item));
        }
        assert_eq!(
            super::fibonacci(64),
            super::BigUint::from(10_610_209_857_723usize)
        );
        assert_eq!(
            super::fibonacci(93),
            super::BigUint::from(12_200_160_415_121_876_738usize)
        );
    }

    #[test]
    fn both_fibonacci() {
        let l = super::FIBONACCI_CACHE.len();
        for x in l - 3..l + 7 {
            assert_eq!(super::fibonacci(x), super::fibonacci_raw(x));
        }
    }
}
