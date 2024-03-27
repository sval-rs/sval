use sval_derive::*;

#[derive(Value)]
#[sval(dynamic, unlabeled_variants)]
pub enum Enum {
    A,
    B,
    C,
}

fn main() {

}