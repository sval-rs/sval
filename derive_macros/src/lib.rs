#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;
extern crate core;

mod attr;
mod bound;
mod derive;
mod index;
mod label;
mod stream;
mod tag;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Value, attributes(sval))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive::derive_value(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(ValueRef, attributes(sval))]
pub fn derive_value_ref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive::derive_value_ref(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
