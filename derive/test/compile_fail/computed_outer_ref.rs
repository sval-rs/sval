use sval_derive::*;

#[derive(Value)]
#[sval(ref)]
pub struct Data<'a> {
    #[sval(computed)]
    #[sval(outer_ref)]
    value: &'a str,
}

fn main() {}
