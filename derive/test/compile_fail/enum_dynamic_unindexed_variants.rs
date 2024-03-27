use sval_derive::*;

#[derive(Value)]
#[sval(dynamic, unindexed_variants)]
pub enum Enum {
    A,
    B,
    C,
}

fn main() {

}