/*!
Implementation details for `sval_derive`.
*/

/*
This library provides a single proc-macro derive that generates `sval::Value` implementations for structs, enums, and newtypes.
It optionally also generates an equivalent `ValueRef` implementation when `#[sval(ref)]` is used.

## Derive expansion

The `derive()` function in the `derive` module classifies the input type and delegates to a specialized handler:

- unit structs become tags.
- single-field tuples become tagged wrappers (or transparent passthroughs).
- multi-field structs become records or tuples, enums become enum streams.
- void enums emit empty matches.

## Attribute parsing

The `attr` module handles `#[sval(...)]` attributes.
Each attribute key has a dedicated struct implementing the `SvalAttribute` trait, which parses the attached expression or literal into a strongly typed result.
Attributes are validated against an allowlist per context (struct, enum, field, etc) to produce clear errors for typos or misplaced attributes.

## Generics and lifetimes

The `bound` module constructs `where` clauses by adding `T: sval::Value` bounds for each type parameter.
The `lifetime` module parses explicit lifetime specifications (with optional bounds) for `ValueRef` derives.
When `#[sval(ref)]` is used without an explicit lifetime, the macro infers it from the type's single lifetime parameter.

## Field streaming strategies

The `value_trait` module defines the `ImplStrategy` trait with two implementations:

- `ImplValue` for `sval::Value`.
- `ImplValueRef` for `sval_ref::ValueRef`.
*/

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
mod lifetime;
mod stream;
mod tag;
mod value_trait;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Value, attributes(sval))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive::derive(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
