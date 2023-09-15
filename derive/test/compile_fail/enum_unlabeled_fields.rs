use sval_derive::*;

#[derive(Value)]
#[sval(unlabeled_fields)]
pub enum Enum {
    A,
    B,
    C,
}

fn main() {

}