/*!
Values are Rust structures that represent a single instance of some datatype.

This example implements the `Value` trait automatically using Rust's `#[derive]` attribute.
*/

#[macro_use]
extern crate sval_derive;

pub mod stream;

#[derive(Value)]
pub struct MyData<'a> {
    id: u64,
    title: &'a str,
}

fn main() -> sval::Result {
    stream(MyData {
        id: 547,
        title: "Some data",
    })?;

    Ok(())
}

fn stream(v: impl sval::Value) -> sval::Result {
    v.stream(&mut stream::simple::MyStream)?;
    println!();

    Ok(())
}
