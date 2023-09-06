mod derive_enum;
mod derive_newtype;
mod derive_struct;
mod derive_unit_struct;
mod derive_void;

use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields};

use self::{
    derive_enum::*, derive_newtype::*, derive_struct::*, derive_unit_struct::*, derive_void::*,
};

pub(crate) fn derive(input: DeriveInput) -> proc_macro2::TokenStream {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            let attrs = UnitStructAttrs::from_attrs(&input.attrs);

            derive_unit_struct(&input.ident, &input.generics, &attrs)
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) if fields.unnamed.len() == 1 => {
            let attrs = NewtypeAttrs::from_attrs(&input.attrs);

            derive_newtype(&input.ident, &input.generics, &fields.unnamed[0], &attrs)
        }
        Data::Struct(DataStruct { ref fields, .. }) => {
            let attrs = StructAttrs::from_attrs(&input.attrs);

            derive_struct(&input.ident, &input.generics, fields, &attrs)
        }
        Data::Enum(DataEnum { ref variants, .. }) if variants.len() == 0 => {
            let attrs = VoidAttrs::from_attrs(&input.attrs);

            derive_void(&input.ident, &input.generics, &attrs)
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let attrs = EnumAttrs::from_attrs(&input.attrs);

            derive_enum(&input.ident, &input.generics, variants.iter(), &attrs)
        }
        _ => panic!("unimplemented"),
    }
}
