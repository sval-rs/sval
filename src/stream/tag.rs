use crate::stream::Ident;

/**
A tag is a carrier for metadata that may be required to interpret a value.
*/
#[derive(Clone, Copy)]
pub enum Tag<'a> {
    Ident {
        kind: Option<Ident<'a>>,
        ident: Ident<'a>,
    },
    Id {
        kind: Option<Ident<'a>>,
        id: u64,
    },
    Full {
        kind: Option<Ident<'a>>,
        ident: Ident<'a>,
        id: u64,
    },
}

impl<'a> Tag<'a> {
    pub fn kind(&self) -> Option<Ident<'a>> {
        match self {
            Tag::Ident { kind, .. } => *kind,
            Tag::Id { kind, .. } => *kind,
            Tag::Full { kind, .. } => *kind,
        }
    }

    pub fn ident(&self) -> Option<Ident<'a>> {
        match self {
            Tag::Ident { ident, .. } => Some(*ident),
            Tag::Id { .. } => None,
            Tag::Full { ident, .. } => Some(*ident),
        }
    }

    pub fn id(&self) -> Option<u64> {
        match self {
            Tag::Ident { .. } => None,
            Tag::Id { id, .. } => Some(*id),
            Tag::Full { id, .. } => Some(*id),
        }
    }
}
