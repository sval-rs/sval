use sval_derive::*;

#[derive(Value)]
#[sval(ref)]
pub struct NoLifetime {
    a: i32,
}

fn main() {}
