use sval_derive::*;

#[derive(Value)]
#[sval(unlabeled_fields)]
pub struct Record {
    #[sval(label = "a")]
    a: i32,
    b: i32,
    c: i32,
}

fn main() {

}