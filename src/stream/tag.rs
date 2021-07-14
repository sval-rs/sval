/*
We have a few new kinds of values we can represent with tags:

let w = W { a: 0, b: 0 }

map_begin()
map_key_field_static("a")
map_value(0)
map_key_field_static("b")
map_value(0)
map_end()

-----

let x = X(0, 0)

seq_begin()
seq_elem(0)
seq_elem(0)
seq_end

-----

let y = Y(0)

i64(0)

-----

let z = Z

none()

-----

let w = E::W { a: 0, b: 0 }

tagged_map_begin_static(Tag::new("E", "W", 0))
map_key_field_static("a")
map_value(0)
map_key_field_static("b")
map_value(0)
tagged_map_end()

-----

E::X(0, 0)

tagged_seq_begin_static(Tag::new("E", "X", 1))
seq_elem(0)
seq_elem(0)
tagged_seq_end()

-----

E::Y(0)

tagged_begin_static(Tag::new("E", "Y", 2))
i64(0)
tagged_end()

-----

E::Z

tag_static(Tag::new("E", "Z", 3))
*/

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
    }
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
