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
    TokenStream::from(derive::derive(parse_macro_input!(input as DeriveInput)))
}
