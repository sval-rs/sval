use proc_macro2::TokenStream;
use syn::{Attribute, Field, Generics, Ident};

use crate::{attr, bound::where_clause_with_bound, lifetime::RefLifetime};

/// Controls how a field is streamed based on attributes and trait context
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum FieldCodegen {
    OuterRef,
    InnerRef,
    Computed,
}

pub(crate) fn field_codegen(attrs: &[Attribute]) -> syn::Result<Option<FieldCodegen>> {
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

/// Wraps a generated stream body in the correct impl block
/// and provides the default field codegen strategy
pub(crate) trait ImplStrategy {
    fn quote_stream_value(
        &self,
        binding: &Ident,
        codegen: Option<FieldCodegen>,
    ) -> syn::Result<TokenStream>;

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
    fn quote_stream_value(
        &self,
        binding: &Ident,
        codegen: Option<FieldCodegen>,
    ) -> syn::Result<TokenStream> {
        Ok(match codegen.unwrap_or(FieldCodegen::OuterRef) {
            FieldCodegen::OuterRef => quote!(stream.value(#binding)),
            FieldCodegen::InnerRef => quote!(stream.value(#binding)),
            FieldCodegen::Computed => quote!(stream.value_computed(#binding)),
        })
    }

    fn quote_impl(
        &self,
        ident: &Ident,
        generics: &Generics,
        stream_body: TokenStream,
    ) -> syn::Result<TokenStream> {
        let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

        // Add Value bound to all type parameters
        let bounded_where_clause = where_clause_with_bound(generics, parse_quote!(sval::Value));

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
    pub(crate) inner_ref_fields: Vec<syn::Type>,
}

impl ImplValueRef {
    pub(crate) fn new(lifetime: RefLifetime, inner_ref_fields: Vec<syn::Type>) -> Self {
        Self {
            lifetime,
            inner_ref_fields,
        }
    }
}

impl ImplStrategy for ImplValueRef {
    fn quote_stream_value(
        &self,
        binding: &Ident,
        codegen: Option<FieldCodegen>,
    ) -> syn::Result<TokenStream> {
        Ok(match codegen.unwrap_or(FieldCodegen::Computed) {
            FieldCodegen::OuterRef => quote!(stream.value(*#binding)),
            FieldCodegen::InnerRef => {
                quote!(sval_derive::extensions::r#ref::stream_ref(stream, #binding))
            }
            FieldCodegen::Computed => quote!(stream.value_computed(#binding)),
        })
    }

    fn quote_impl(
        &self,
        ident: &Ident,
        generics: &Generics,
        stream_body: TokenStream,
    ) -> syn::Result<TokenStream> {
        let lifetime = &self.lifetime.lifetime;

        // TODO: Add the lifetime and its bounds

        // Build impl generics: add the ValueRef lifetime with optional bounds
        let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

        // Merge where clauses
        let mut bounded_where_clause = where_clause_with_bound(generics, parse_quote!(sval::Value));

        // Add ValueRef bounds for inner_ref fields
        // We can unconditionally add the full field type to the where clause
        // This works for both concrete types (Inner<'a>: ValueRef<'a>) and generics (T: ValueRef<'a>)
        for field_type in &self.inner_ref_fields {
            bounded_where_clause.predicates.push(
                parse_quote!(#field_type: sval_derive::extensions::r#ref::ValueRef<#lifetime>),
            );
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
            impl #impl_generics sval_derive::extensions::r#ref::ValueRef<#lifetime> for #ident #ty_generics #bounded_where_clause {
                #stream_fn
            }
        })
    }
}

/// Collect field types that have the inner_ref attribute
pub(crate) fn collect_inner_ref_field_types<'a, I>(fields: I) -> syn::Result<Vec<syn::Type>>
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
