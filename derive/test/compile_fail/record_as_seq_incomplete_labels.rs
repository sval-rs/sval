use sval_derive::*;

#[derive(Value)]
#[sval(unindexed_values, unlabeled_values)]
pub struct Record {
    #[sval(label = "a")]
    a: i32,
    b: i32,
    c: i32,
}

fn main() {

}