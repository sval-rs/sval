use crate::stream::Ident;

/**
A tag is an identifier for an enum variant.
*/
#[derive(Clone, Copy)]
pub enum Tag<'a> {
    Named {
        ty: Option<Ident<'a>>,
        name: Ident<'a>,
    },
    Indexed {
        ty: Option<Ident<'a>>,
        index: u32,
    },
    Full {
        ty: Option<Ident<'a>>,
        name: Ident<'a>,
        index: u32,
    },
}

impl<'a> Tag<'a> {
    pub fn ty(&self) -> Option<Ident<'a>> {
        match self {
            Tag::Named { ty, .. } => *ty,
            Tag::Indexed { ty, .. } => *ty,
            Tag::Full { ty, .. } => *ty,
        }
    }

    pub fn name(&self) -> Option<Ident<'a>> {
        match self {
            Tag::Named { name, .. } => Some(*name),
            Tag::Indexed { .. } => None,
            Tag::Full { name, .. } => Some(*name),
        }
    }

    pub fn index(&self) -> Option<u32> {
        match self {
            Tag::Named { .. } => None,
            Tag::Indexed { index, .. } => Some(*index),
            Tag::Full { index, .. } => Some(*index),
        }
    }
}
