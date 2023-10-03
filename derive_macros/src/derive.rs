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
        _ => panic!("unsupported container type"),
    }
}

fn impl_tokens(
    impl_generics: syn::ImplGenerics,
    ident: &syn::Ident,
    ty_generics: syn::TypeGenerics,
    bounded_where_clause: &syn::WhereClause,
    stream_body: proc_macro2::TokenStream,
    tag_body: Option<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    let stream_fn = quote!(
        fn stream<'sval, __SvalStream: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut __SvalStream) -> sval::Result {
            #stream_body
        }
    );

    let tag_fn = if let Some(tag_body) = tag_body {
        quote!(
            fn tag(&self) -> Option<sval::Tag> {
                #tag_body
            }
        )
    } else {
        quote!()
    };

    quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                #stream_fn

                #tag_fn
            }
        };
    }
}
