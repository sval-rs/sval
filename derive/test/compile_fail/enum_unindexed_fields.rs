use sval_derive::*;

#[derive(Value)]
#[sval(unindexed_values)]
pub enum Enum {
    A,
    B,
    C,
}

fn main() {

}