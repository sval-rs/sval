#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

mod attr;
mod bound;
mod value;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Value, attributes(sval))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    value::derive(parse_macro_input!(input as DeriveInput))
}
