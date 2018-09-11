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

#[macro_use]
extern crate criterion;
extern crate fibonacci_common;

use criterion::Criterion;
use fibonacci_common::{fibonacci, fibonacci_raw};

fn benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci       5", |b| b.iter(|| fibonacci(5)));
    c.bench_function("fibonacci raw   5", |b| b.iter(|| fibonacci_raw(5)));
    c.bench_function("fibonacci     500", |b| b.iter(|| fibonacci(500)));
    c.bench_function("fibonacci raw 500", |b| b.iter(|| fibonacci_raw(500)));
    c.bench_function("fibonacci     520", |b| b.iter(|| fibonacci(520)));
    c.bench_function("fibonacci raw 520", |b| b.iter(|| fibonacci_raw(520)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
