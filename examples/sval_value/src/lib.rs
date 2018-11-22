use sval::value::{
    self,
    Value,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Id(u64);

impl Id {
    pub fn new(id: u64) -> Self {
        Id(id)
    }
}

impl Value for Id {
    fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
        stream.u64(self.0)
    }
}
