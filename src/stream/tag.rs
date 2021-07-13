#[derive(Clone, Copy)]
pub struct Tag<'a> {
    ident: &'a str,
    index: u32,
}

pub fn tag<'a>(ident: &'a str, index: u32) -> Tag<'a> {
    Tag::new(ident, index)
}

impl<'a> Tag<'a> {
    pub fn new(ident: &'a str, index: u32) -> Self {
        Tag { ident, index }
    }

    pub fn ident(&self) -> &str {
        self.ident
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}
