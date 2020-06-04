/*!
Derive `sval::Value`.

Use the `derive` feature of `sval` instead of depending on this library directly:

```toml,no_run
[dependencies.sval]
features = ["derive"]
```
*/

/*
This `derive` implementation has been shamelessly lifted from dtolnay's `miniserde`:
https://github.com/dtolnay/miniserde
*/

#![doc(html_root_url = "https://docs.rs/sval_derive/0.5.2")]
#![recursion_limit = "128"]

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

mod attr;
mod bound;
mod value;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Value, attributes(sval))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    value::derive(parse_macro_input!(input as DeriveInput))
}
