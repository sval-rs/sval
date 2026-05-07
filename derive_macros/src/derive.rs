/*!
Type classification and dispatch for derive expansion.
*/

mod derive_enum;
mod derive_newtype;
mod derive_struct;
mod derive_unit_struct;
mod derive_void;

use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields};

use crate::derive::{
    derive_enum::*, derive_newtype::*, derive_struct::*, derive_unit_struct::*, derive_void::*,
};

/**
Inspect the `DeriveInput` and route to a specialized handler based on the data kind.

- **Unit structs:** (`derive_unit_struct`) emit a `stream.tag()` call.
- **Newtypes:** (`derive_newtype`) single-field tuple, emit a tagged wrapper or transparent passthrough.
- **Structs:** (`derive_struct`) named or unnamed fields, emit a record/tuple with indexed and labeled values.
- **Void enums:** (`derive_void`) zero variants, emit an empty match.
- **Enums:** (`derive_enum`) one or more variants, emit an enum stream wrapping variant-level tag/newtype/record-tuple streams.
*/
pub(crate) fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let impl_tokens = match &input.data {
        // Unit struct
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            let attrs = UnitStructAttrs::from_attrs(&input.attrs)?;

            derive_unit_struct(&input.ident, &input.generics, &attrs)?
        }
        // Newtype
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) if fields.unnamed.len() == 1 => {
            let attrs = NewtypeAttrs::from_attrs(&input.attrs)?;

            derive_newtype(&input.ident, &input.generics, &fields.unnamed[0], &attrs)?
        }
        // Record or tuple struct
        Data::Struct(DataStruct { ref fields, .. }) => {
            let attrs = StructAttrs::from_attrs(&input.attrs)?;

            derive_struct(&input.ident, &input.generics, fields, &attrs)?
        }
        // Void
        Data::Enum(DataEnum { ref variants, .. }) if variants.len() == 0 => {
            let attrs = VoidAttrs::from_attrs(&input.attrs)?;

            derive_void(&input.ident, &input.generics, &attrs)?
        }
        // Enum
        Data::Enum(DataEnum { variants, .. }) => {
            let attrs = EnumAttrs::from_attrs(&input.attrs)?;

            derive_enum(&input.ident, &input.generics, variants.iter(), &attrs)?
        }
        Data::Union(u) => {
            return Err(syn::Error::new(
                u.union_token.span(),
                "unions are not supported for sval derive",
            ));
        }
    };

    Ok(quote! {
        const _: () = {
            extern crate sval;

            #impl_tokens
        };
    })
}
