#[derive(Clone, Copy)]
pub struct Tag<'a> {
    name: Option<&'a str>,
    value: Option<&'a str>,
    index: Option<u32>,
}

impl<'a> Tag<'a> {
    pub fn named(name: &'a str) -> Self {
        Tag {
            name: Some(name),
            value: None,
            index: None,
        }
    }

    pub fn variant(value: &'a str, index: u32) -> Self {
        Tag {
            name: None,
            value: Some(value),
            index: Some(index),
        }
    }

    pub fn named_variant(name: &'a str, value: &'a str, index: u32) -> Self {
        Tag {
            name: Some(name),
            value: Some(value),
            index: Some(index),
        }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn value(&self) -> Option<&'a str> {
        self.value
    }

    pub fn index(&self) -> Option<u32> {
        self.index
    }
}
