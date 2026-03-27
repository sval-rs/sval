use syn::{Attribute, Field, Generics, Ident, Path};

use crate::{
    attr, bound,
    derive::impl_tokens,
    index::{Index, IndexAllocator, IndexValue},
    label::{label_or_ident, LabelValue},
    stream::stream_newtype,
    tag::quote_optional_tag_owned,
};

pub(crate) struct NewtypeAttrs {
    tag: Option<Path>,
    label: Option<LabelValue>,
    index: Option<IndexValue>,
    transparent: bool,
}

impl NewtypeAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        attr::check(
            "newtype",
            &[
                &attr::TagAttr,
                &attr::LabelAttr,
                &attr::IndexAttr,
                &attr::TransparentAttr,
            ],
            attrs,
        )?;

        let tag = attr::get_unchecked("newtype", attr::TagAttr, attrs)?;
        let label = attr::get_unchecked("newtype", attr::LabelAttr, attrs)?;
        let index = attr::get_unchecked("newtype", attr::IndexAttr, attrs)?;
        let transparent =
            attr::get_unchecked("newtype", attr::TransparentAttr, attrs)?.unwrap_or(false);

        if transparent {
            if tag.is_some() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "transparent values cannot have tags",
                ));
            }
            if label.is_some() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "transparent values cannot have labels",
                ));
            }
            if index.is_some() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "transparent values cannot have indexes",
                ));
            }
        }

        Ok(NewtypeAttrs {
            tag,
            label,
            index,
            transparent,
        })
    }

    pub(crate) fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    pub(crate) fn label(&self) -> Option<LabelValue> {
        self.label.clone()
    }

    pub(crate) fn index(&self) -> Option<Index> {
        self.index.clone().map(IndexAllocator::const_index_of)
    }

    pub(crate) fn transparent(&self) -> bool {
        self.transparent
    }
}

pub(crate) fn derive_newtype<'a>(
    ident: &Ident,
    generics: &Generics,
    field: &Field,
    attrs: &NewtypeAttrs,
) -> syn::Result<proc_macro2::TokenStream> {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_newtype(
        quote!(#ident),
        field,
        attrs.tag(),
        Some(label_or_ident(attrs.label(), ident)),
        attrs.index(),
        attrs.transparent(),
    )?;

    let tag = quote_optional_tag_owned(attrs.tag());

    Ok(impl_tokens(
        impl_generics,
        ident,
        ty_generics,
        &bounded_where_clause,
        quote!({
            match self {
                #match_arm
            }

            sval::__private::result::Result::Ok(())
        }),
        Some(tag),
    ))
}
