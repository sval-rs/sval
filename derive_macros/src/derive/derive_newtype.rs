use syn::{Attribute, Field, Generics, Ident, Path};

use crate::{
    attr::{self, RefAttrValue},
    derive::{infer_ref_lifetime, ImplStrategy, ImplValue, ImplValueRef},
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
    ref_attr: Option<RefAttrValue>,
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
                &attr::RefAttr,
            ],
            attrs,
        )?;

        let tag = attr::get("newtype", attr::TagAttr, attrs)?;
        let label = attr::get("newtype", attr::LabelAttr, attrs)?;
        let index = attr::get("newtype", attr::IndexAttr, attrs)?;
        let transparent = attr::get("newtype", attr::TransparentAttr, attrs)?.unwrap_or(false);
        let ref_attr = attr::get("newtype", attr::RefAttr, attrs)?;

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
            ref_attr,
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

    pub(crate) fn value_ref_lifetime(&self) -> Option<&RefAttrValue> {
        self.ref_attr.as_ref()
    }
}

pub(crate) fn derive_newtype<'a>(
    ident: &Ident,
    generics: &Generics,
    field: &Field,
    attrs: &NewtypeAttrs,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut impl_blocks = vec![ImplValue::new(Some(quote_optional_tag_owned(attrs.tag()))).boxed()];

    if let Some(ref_attr) = attrs.value_ref_lifetime() {
        impl_blocks.push(
            ImplValueRef::new(
                match ref_attr.lifetime() {
                    Some(lt) => lt.clone(),
                    None => infer_ref_lifetime(generics)?,
                },
                vec![],
            )
            .boxed(),
        );
    }

    let mut impl_tokens = Vec::new();
    for block in impl_blocks {
        let match_arm = stream_newtype(
            quote!(#ident),
            field,
            &*block,
            attrs.tag(),
            Some(label_or_ident(attrs.label(), ident)),
            attrs.index(),
            attrs.transparent(),
        )?;

        impl_tokens.push(block.quote_impl(
            ident,
            generics,
            quote!({
                match self {
                    #match_arm
                }

                sval::__private::result::Result::Ok(())
            }),
        )?);
    }

    Ok(quote!(#(#impl_tokens)*))
}
