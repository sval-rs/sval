mod derive_enum;
mod derive_newtype;
mod derive_struct;
mod derive_unit_struct;
mod derive_void;

use syn::{spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields};

use self::{
    derive_enum::*, derive_newtype::*, derive_struct::*, derive_unit_struct::*, derive_void::*,
};

pub(crate) fn derive_value(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    derive_inner::<ValueTokens>(input)
}

pub(crate) fn derive_value_ref(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    derive_inner::<ValueRefTokens>(input)
}

pub(crate) fn derive_inner<T: ImplTokens>(
    input: DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    Ok(match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            let attrs = UnitStructAttrs::from_attrs(&input.attrs)?;

            derive_unit_struct::<T>(&input.ident, &input.generics, &attrs)?
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) if fields.unnamed.len() == 1 => {
            let attrs = NewtypeAttrs::from_attrs(&input.attrs)?;

            derive_newtype::<T>(&input.ident, &input.generics, &fields.unnamed[0], &attrs)?
        }
        Data::Struct(DataStruct { ref fields, .. }) => {
            let attrs = StructAttrs::from_attrs(&input.attrs)?;

            derive_struct::<T>(&input.ident, &input.generics, fields, &attrs)?
        }
        Data::Enum(DataEnum { ref variants, .. }) if variants.len() == 0 => {
            let attrs = VoidAttrs::from_attrs(&input.attrs)?;

            derive_void::<T>(&input.ident, &input.generics, &attrs)?
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let attrs = EnumAttrs::from_attrs(&input.attrs)?;

            derive_enum::<T>(&input.ident, &input.generics, variants.iter(), &attrs)?
        }
        Data::Union(u) => {
            return Err(syn::Error::new(
                u.union_token.span(),
                "unions are not supported for sval derive",
            ));
        }
    })
}

pub(crate) trait ImplTokens {
    fn impl_tokens(
        impl_generics: syn::ImplGenerics,
        ident: &syn::Ident,
        ty_generics: syn::TypeGenerics,
        bounded_where_clause: &syn::WhereClause,
        lifetime: Option<LifetimeValue>,
        stream_body: proc_macro2::TokenStream,
        tag_body: Option<proc_macro2::TokenStream>,
    ) -> proc_macro2::TokenStream;
}

pub(crate) struct ValueTokens;

impl ImplTokens for ValueTokens {
    fn impl_tokens(
        impl_generics: syn::ImplGenerics,
        ident: &syn::Ident,
        ty_generics: syn::TypeGenerics,
        bounded_where_clause: &syn::WhereClause,
        _lifetime: Option<LifetimeValue>,
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
                fn tag(&self) -> sval::__private::option::Option<sval::Tag> {
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
}

pub(crate) struct ValueRefTokens;

impl ImplTokens for ValueRefTokens {
    fn impl_tokens(
        impl_generics: syn::ImplGenerics,
        ident: &syn::Ident,
        ty_generics: syn::TypeGenerics,
        bounded_where_clause: &syn::WhereClause,
        lifetime: Option<LifetimeValue>,
        stream_body: proc_macro2::TokenStream,
        _tag_body: Option<proc_macro2::TokenStream>,
    ) -> proc_macro2::TokenStream {
        // ValueRef doesn't have a tag method, so we ignore tag_body
        let (lifetime, merged_where_clause) = if let Some(lifetime) = lifetime {
            match lifetime {
                LifetimeValue::Named(lifetime) => (quote!(#lifetime), bounded_where_clause.clone()),
                LifetimeValue::Bounded { lifetime, bounds } => {
                    let mut merged_where_clause = bounded_where_clause.clone();
                    merged_where_clause
                        .predicates
                        .push(parse_quote!(#lifetime: #bounds));

                    (quote!(#lifetime), merged_where_clause)
                }
            }
        } else {
            (quote!('sval), bounded_where_clause.clone())
        };

        let stream_fn = quote!(
            fn stream_ref<__S: sval::Stream<#lifetime> + ?Sized>(&self, stream: &mut __S) -> sval::Result {
                #stream_body
            }
        );

        quote! {
            const _: () = {
                extern crate sval;

                impl #impl_generics sval_derive::extensions::value_ref::ValueRef<#lifetime> for #ident #ty_generics #merged_where_clause {
                    #stream_fn
                }
            };
        }
    }
}

#[derive(Clone)]
pub(crate) enum LifetimeValue {
    Named(syn::Lifetime),
    Bounded {
        lifetime: syn::Lifetime,
        bounds: syn::punctuated::Punctuated<syn::Lifetime, syn::token::Plus>,
    },
}

impl syn::parse::Parse for LifetimeValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse the lifetime name
        let lifetime: syn::Lifetime = input.parse()?;

        // Check if there's a colon for bounds
        if input.peek(syn::token::Colon) {
            // Parse the colon
            let _colon: syn::token::Colon = input.parse()?;

            // Parse the bounds (e.g., 'a + 'b)
            let bounds: syn::punctuated::Punctuated<syn::Lifetime, syn::token::Plus> =
                input.parse_terminated(syn::Lifetime::parse, syn::token::Plus)?;

            Ok(LifetimeValue::Bounded { lifetime, bounds })
        } else {
            Ok(LifetimeValue::Named(lifetime))
        }
    }
}
