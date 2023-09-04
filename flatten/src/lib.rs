struct IndexAllocator {
    initial_offset: usize,
    current_offset: usize,
}

impl IndexAllocator {
    fn start_from(offset: usize) -> Self {
        IndexAllocator {
            initial_offset: offset,
            current_offset: offset,
        }
    }

    fn next_begin(&mut self, incoming: Option<&sval::Index>) -> sval::Index {
        match incoming {
            // If there's an incoming tag then merge it into the current set
            Some(incoming) => match (incoming.tag(), incoming.to_usize()) {
                // If the incoming tag is a value offset then increment it by our starting point
                (Some(&sval::tags::VALUE_OFFSET), Some(incoming)) => {
                    sval::Index::new(incoming + self.initial_offset)
                        .with_tag(&sval::tags::VALUE_OFFSET)
                }
                // If the incoming tag is not a value offset then just use it directly
                _ => incoming.clone(),
            },
            // If there's no incoming tag then construct one
            None => sval::Index::new(self.current_offset).with_tag(&sval::tags::VALUE_OFFSET),
        }
    }

    fn next_end(&mut self, incoming: Option<&sval::Index>) -> sval::Index {
        let index = self.next_begin(incoming);
        self.current_offset += 1;

        index
    }
}

mod record;
mod record_tuple;
mod tuple;

pub use self::{record::*, record_tuple::*, tuple::*};
