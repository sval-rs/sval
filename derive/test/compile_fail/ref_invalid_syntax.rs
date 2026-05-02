use sval_derive::*;

#[derive(Value)]
#[sval(ref = "invalid")]
pub struct InvalidSyntax<'a> {
    a: &'a i32,
}

fn main() {}
