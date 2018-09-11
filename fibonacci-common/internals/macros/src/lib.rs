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

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

extern crate fibonacci_functions;

use fibonacci_functions::fibonacci;

struct ConstructFibonacci {
    pub name: syn::Ident,
    pub max: syn::LitInt,
}

impl syn::synom::Synom for ConstructFibonacci {
    named!(parse -> Self, do_parse!(
        name: syn!(syn::Ident) >>
        punct!(,) >>
        max: syn!(syn::LitInt) >>
        (ConstructFibonacci { name, max })
    ));
}

macro_rules! padding_array {
    ($bu:expr, $len:ident) => {{
        let be = $bu.to_bytes_be();
        let mut q = quote!();
        for _ in 0..$len - be.len() {
            q = quote!(#q 0,);
        }
        for i in &be[..] {
            q = quote!(#q #i,);
        }
        q
    }};
}

#[proc_macro]
pub fn construct_fibonacci_cache(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = {
        let ConstructFibonacci { name, max } = syn::parse2(input).unwrap();
        let m = max.value() as usize;
        let mut x = (fibonacci(0), fibonacci(1));
        let l = fibonacci(m).to_bytes_be().len();
        let mut vec_inner = {
            let t0 = padding_array!(x.0, l);
            let t1 = padding_array!(x.1, l);
            quote!([#t0], [#t1])
        };
        let mut i = 2usize;
        while i < m {
            i += 1;
            x.0 += &x.1;
            x = (x.1, x.0);
            let tmp = padding_array!(x.1, l);
            vec_inner = quote!(#vec_inner, [#tmp]);
        }
        quote!(pub static #name: [[u8; #l]; #m] = [#vec_inner];)
    };
    output.into()
}
