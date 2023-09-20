/*!
`#![derive(Value)]`.

This library hosts a custom-derive to simplify implementing `sval::Value`.

# Structs

Container attributes:

- `#[sval(tag = "path::to::TAG")]`: Set a tag on the struct. No tag is used by default.
- `#[sval(label = "text")]`: Set a label on the struct. The identifier of the struct is used by default.
- `#[sval(index = 1)]`: Set an index on the struct. No index is used by default.
- `#[sval(unlabeled_fields)]`: Specify that all fields should be unlabeled. This will stream the struct as a tuple.
If `#[sval(unindexed_fields)]` is also specified then it will stream the struct as a sequence.
- `#[sval(unindexed_fields]`: Specify that all fields should be unindexed. This will stream the struct as a record.
If `#[sval(unlabeled_fields)]` is also specified then it will stream the struct as a sequence.

Field attributes:

- `#[sval(tag = "path::to::TAG")]`: Set a tag on the struct field itself. No tag is used by default.
If you want to use a tag to signal that the field's value has a particular property then use `#[sval(data_tag)]`.
- `#[sval(data_tag = "path::to::TAG")]`: Set a tag on the struct field's value. No tag is used by default.
- `#[sval(label = "text")]`: Set a label on the struct field. The identifier of the field is used by default.
- `#[sval(index = 1)]`: Set an index on the struct field. The zero-based offset of the field is used by default.
- `#[sval(skip)]`: Skip a field.
- `#[sval(flatten)]`: Flatten the field onto the struct. This attribute requires the `flatten` Cargo feature.

# Newtypes

Container attributes:

- `#[sval(tag = "path::to::TAG")]`: Set a tag on the newtype. No tag is used by default.
- `#[sval(label = "text")]`: Set a label on the newtype. The identifier of the newtype is used by default.
- `#[sval(index = 1)]`: Set an index on the newtype. No index is used by default.
- `#[sval(transparent)]`: Stream the newtype as its underlying field without wrapping it.

# Enums

Container attributes:

- `#[sval(tag = "path::to::TAG")]`: Set a tag on the enum. No tag is used by default.
- `#[sval(label = "text")]`: Set a label on the enum. The identifier of the enum is used by default.
- `#[sval(index = 1)]`: Set an index on the enum. No index is used by default.
- `#[sval(dynamic)]`: Stream the variant without wrapping it in an enum.

Variant attributes:

- `#[sval(tag = "path::to::TAG")]`: Set a tag on the enum variant itself. No tag is used by default.
- `#[sval(label = "text")]`: Set a label on the enum variant. The identifier of the variant is used by default.
- `#[sval(index = 1)]`: Set an index on the enum variant. The zero-based offset of the variant is used by default.
*/

#[doc(inline)]
pub use sval_derive_macros::*;

pub mod extensions {
    #[cfg(feature = "flatten")]
    pub use sval_flatten as flatten;
}
