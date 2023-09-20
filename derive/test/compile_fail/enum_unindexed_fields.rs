use sval_derive::*;

#[derive(Value)]
#[sval(unindexed_fields)]
pub enum Enum {
    A,
    B,
    C,
}

fn main() {

}