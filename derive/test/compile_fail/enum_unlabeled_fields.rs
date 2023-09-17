use sval_derive::*;

#[derive(Value)]
#[sval(unlabeled_values)]
pub enum Enum {
    A,
    B,
    C,
}

fn main() {

}