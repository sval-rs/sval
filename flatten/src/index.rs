use sval::Index;

pub(crate) struct IndexAllocator {
    initial_offset: isize,
    current_offset: isize,
}

impl IndexAllocator {
    pub(crate) fn start_from(offset: isize) -> Self {
        IndexAllocator {
            initial_offset: offset,
            current_offset: offset,
        }
    }

    pub(crate) fn next_begin(&mut self, incoming: Option<&Index>) -> Index {
        match incoming {
            // If there's an incoming tag then merge it into the current set
            Some(incoming) => match (incoming.tag(), incoming.to_isize()) {
                // If the incoming tag is a value offset then increment it by our starting point
                (Some(&sval::tags::VALUE_OFFSET), Some(incoming)) => {
                    Index::new_isize(incoming + self.initial_offset)
                        .with_tag(&sval::tags::VALUE_OFFSET)
                }
                // If the incoming tag is not a value offset then just use it directly
                _ => incoming.clone(),
            },
            // If there's no incoming tag then construct one
            None => Index::new_isize(self.current_offset).with_tag(&sval::tags::VALUE_OFFSET),
        }
    }

    pub(crate) fn next_end(&mut self, incoming: Option<&Index>) -> Index {
        let index = self.next_begin(incoming);
        self.current_offset = index.to_isize().unwrap_or(self.current_offset) + 1;

        index
    }

    pub(crate) fn current_offset(&self) -> isize {
        self.current_offset
    }
}
