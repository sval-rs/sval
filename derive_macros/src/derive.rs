mod derive_enum;
mod derive_newtype;
mod derive_struct;
mod derive_unit_struct;
mod derive_void;

use proc_macro2::TokenStream;
use syn::{
    spanned::Spanned, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Generics,
    Ident, Type,
};

use crate::{
    attr,
    derive::{
        derive_enum::*, derive_newtype::*, derive_struct::*, derive_unit_struct::*, derive_void::*,
    },
    lifetime::RefLifetime,
};

pub(crate) fn derive(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let impl_tokens = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            let attrs = derive_unit_struct::UnitStructAttrs::from_attrs(&input.attrs)?;

            derive_unit_struct(&input.ident, &input.generics, &attrs)?
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) if fields.unnamed.len() == 1 => {
            let attrs = derive_newtype::NewtypeAttrs::from_attrs(&input.attrs)?;

            derive_newtype(&input.ident, &input.generics, &fields.unnamed[0], &attrs)?
        }
        Data::Struct(DataStruct { ref fields, .. }) => {
            let attrs = StructAttrs::from_attrs(&input.attrs)?;

            derive_struct(&input.ident, &input.generics, fields, &attrs)?
        }
        Data::Enum(DataEnum { ref variants, .. }) if variants.len() == 0 => {
            let attrs = derive_void::VoidAttrs::from_attrs(&input.attrs)?;

            derive_void(&input.ident, &input.generics, &attrs)?
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let attrs = derive_enum::EnumAttrs::from_attrs(&input.attrs)?;

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

/// Controls how a field is streamed based on attributes and trait context
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum FieldCodegen {
    OuterRef,
    InnerRef,
    Computed,
}

fn field_codegen(attrs: &[Attribute]) -> syn::Result<Option<FieldCodegen>> {
    let outer_ref = attr::get("struct field", attr::OuterRefAttr, attrs)?.unwrap_or(false);
    let inner_ref = attr::get("struct field", attr::InnerRefAttr, attrs)?.unwrap_or(false);
    let computed = attr::get("struct field", attr::ComputedAttr, attrs)?.unwrap_or(false);

    let specified_count = [outer_ref, inner_ref, computed]
        .iter()
        .filter(|b| **b)
        .count();

    if specified_count > 1 {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "only one of `outer_ref`, `inner_ref`, or `computed` can be specified",
        ));
    }

    Ok(if outer_ref {
        Some(FieldCodegen::OuterRef)
    } else if inner_ref {
        Some(FieldCodegen::InnerRef)
    } else if computed {
        Some(FieldCodegen::Computed)
    } else {
        None
    })
}

pub(crate) fn quote_stream_value(
    binding: &Ident,
    attrs: &[Attribute],
    default: FieldCodegen,
) -> syn::Result<TokenStream> {
    Ok(match field_codegen(attrs)?.unwrap_or(default) {
        FieldCodegen::OuterRef => quote!(stream.value(#binding)),
        FieldCodegen::InnerRef => quote!(sval_ref::stream_ref(stream, #binding)),
        FieldCodegen::Computed => quote!(stream.value_computed(#binding)),
    })
}

/// Wraps a generated stream body in the correct impl block
/// and provides the default field codegen strategy
pub(crate) trait ImplStrategy {
    /// Return the default field codegen strategy for this trait
    fn default_field_codegen(&self) -> FieldCodegen;

    /// Wrap the stream body in an impl block
    fn quote_impl(
        &self,
        ident: &Ident,
        generics: &Generics,
        stream_body: TokenStream,
    ) -> syn::Result<TokenStream>;

    fn boxed(self) -> Box<dyn ImplStrategy>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

/// Generates impl block for sval::Value
pub(crate) struct ImplValue {
    tag_body: Option<TokenStream>,
}

impl ImplValue {
    pub(crate) fn new(tag_body: Option<TokenStream>) -> Self {
        Self { tag_body }
    }
}

impl ImplStrategy for ImplValue {
    fn default_field_codegen(&self) -> FieldCodegen {
        FieldCodegen::OuterRef
    }

    fn quote_impl(
        &self,
        ident: &Ident,
        generics: &Generics,
        stream_body: TokenStream,
    ) -> syn::Result<TokenStream> {
        let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

        // Add Value bound to all type parameters
        let bounded_where_clause =
            crate::bound::where_clause_with_bound(generics, parse_quote!(sval::Value));

        let stream_fn = quote!(
            fn stream<'sval, __SvalStream: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut __SvalStream,
            ) -> sval::Result {
                #stream_body
            }
        );

        let tag_fn = if let Some(tag_body) = &self.tag_body {
            quote!(
                fn tag(&self) -> sval::__private::option::Option<sval::Tag> {
                    #tag_body
                }
            )
        } else {
            quote!()
        };

        Ok(quote! {
            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                #stream_fn
                #tag_fn
            }
        })
    }
}

/// Generates impl block for sval_ref::ValueRef<'sval>
pub(crate) struct ImplValueRef {
    pub(crate) lifetime: RefLifetime,
    pub(crate) inner_ref_fields: Vec<Type>,
}

impl ImplValueRef {
    pub(crate) fn new(lifetime: RefLifetime, inner_ref_fields: Vec<Type>) -> Self {
        Self {
            lifetime,
            inner_ref_fields,
        }
    }
}

impl ImplStrategy for ImplValueRef {
    fn default_field_codegen(&self) -> FieldCodegen {
        FieldCodegen::Computed
    }

    fn quote_impl(
        &self,
        ident: &Ident,
        generics: &Generics,
        stream_body: TokenStream,
    ) -> syn::Result<TokenStream> {
        let lifetime = &self.lifetime.lifetime;
        let bounds = &self.lifetime.bounds;

        // Build impl generics: add the ValueRef lifetime with optional bounds
        let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

        // Construct the impl generics with lifetime
        let impl_generics = if let Some(bounds) = bounds {
            // Merge the bounds into the impl generics
            quote!(#lifetime #bounds #impl_generics)
        } else {
            quote!(#lifetime #impl_generics)
        };

        // Merge where clauses
        let mut bounded_where_clause =
            crate::bound::where_clause_with_bound(generics, parse_quote!(sval::Value));

        // Add ValueRef bounds for inner_ref fields
        // We can unconditionally add the full field type to the where clause
        // This works for both concrete types (Inner<'a>: ValueRef<'a>) and generics (T: ValueRef<'a>)
        for field_type in &self.inner_ref_fields {
            let bound = quote!(#field_type: sval_ref::ValueRef<#lifetime>);
            bounded_where_clause.predicates.push(parse_quote!(#bound));
        }

        let stream_fn = quote!(
            fn stream_ref<__SvalStream: sval::Stream<#lifetime> + ?Sized>(
                &self,
                stream: &mut __SvalStream,
            ) -> sval::Result {
                #stream_body
            }
        );

        Ok(quote! {
            impl #impl_generics sval_ref::ValueRef<#lifetime> for #ident #ty_generics #bounded_where_clause {
                #stream_fn
            }
        })
    }
}

/// Collect field types that have the inner_ref attribute
pub(crate) fn collect_inner_ref_field_types<'a, I>(fields: I) -> syn::Result<Vec<Type>>
where
    I: Iterator<Item = &'a Field>,
{
    let mut inner_ref_types = Vec::new();

    for field in fields {
        if let Some(FieldCodegen::InnerRef) = field_codegen(&field.attrs)? {
            inner_ref_types.push(field.ty.clone());
        }
    }

    Ok(inner_ref_types)
}

/// Infer a lifetime to use as `'sval` in `ValueRef<'sval>`
pub(crate) fn infer_ref_lifetime(generics: &Generics) -> syn::Result<RefLifetime> {
    let lifetimes: Vec<_> = generics.lifetimes().map(|lt| lt.lifetime.clone()).collect();

    match lifetimes.len() {
        1 => Ok(RefLifetime {
            lifetime: lifetimes[0].clone(),
            bounds: None,
        }),
        0 => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "no lifetime parameter to infer",
        )),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "multiple lifetime parameters: specify which to use with #[sval(ref = \"'lifetime\")]",
        )),
    }
}
