use syn::{Attribute, Field, Generics, Ident, Path};

use crate::{
    attr, bound,
    derive::{self, ImplTokens},
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
    lifetime: Option<derive::LifetimeValue>,
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
                &attr::LifetimeAttr,
            ],
            attrs,
        )?;

        let tag = attr::get("newtype", attr::TagAttr, attrs)?;
        let label = attr::get("newtype", attr::LabelAttr, attrs)?;
        let index = attr::get("newtype", attr::IndexAttr, attrs)?;
        let transparent = attr::get("newtype", attr::TransparentAttr, attrs)?.unwrap_or(false);
        let lifetime = attr::get("newtype", attr::LifetimeAttr, attrs)?;

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
            lifetime,
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

    pub(crate) fn lifetime(&self) -> Option<derive::LifetimeValue> {
        self.lifetime.clone()
    }
}

pub(crate) fn derive_newtype<'a, T: ImplTokens>(
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

    Ok(T::impl_tokens(
        impl_generics,
        ident,
        ty_generics,
        &bounded_where_clause,
        attrs.lifetime(),
        quote!({
            match self {
                #match_arm
            }

            sval::__private::result::Result::Ok(())
        }),
        Some(tag),
    ))
}
