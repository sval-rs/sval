#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Id(u64);

impl Id {
    pub fn new(id: u64) -> Self {
        Id(id)
    }
}

#[derive(Debug, Serialize)]
pub enum Complex {
    Unit,
    Tuple(i32, i32),
    NewType(i32),
    Struct { a: i32, b: i32 },
}
