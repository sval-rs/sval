use syn::{Attribute, Generics, Ident, Path};

use crate::{
    attr,
    index::{Index, IndexAllocator, IndexValue},
    label::{label_or_ident, LabelValue},
    stream::stream_tag,
    tag::quote_optional_tag_owned,
    value_trait::{ImplStrategy, ImplValue},
};

pub(crate) struct UnitStructAttrs {
    tag: Option<Path>,
    label: Option<LabelValue>,
    index: Option<IndexValue>,
}

impl UnitStructAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        attr::check(
            "unit struct",
            &[&attr::TagAttr, &attr::LabelAttr, &attr::IndexAttr],
            attrs,
        )?;

        let tag = attr::get("unit struct", attr::TagAttr, attrs)?;
        let label = attr::get("unit struct", attr::LabelAttr, attrs)?;
        let index = attr::get("unit struct", attr::IndexAttr, attrs)?;

        Ok(UnitStructAttrs { tag, label, index })
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
}

pub(crate) fn derive_unit_struct<'a>(
    ident: &Ident,
    generics: &Generics,
    attrs: &UnitStructAttrs,
) -> syn::Result<proc_macro2::TokenStream> {
    let match_arm = stream_tag(
        quote!(_),
        attrs.tag(),
        Some(label_or_ident(attrs.label(), ident)),
        attrs.index(),
    )?;

    ImplValue::new(Some(quote_optional_tag_owned(attrs.tag()))).quote_impl(
        ident,
        generics,
        quote!({
            match self {
                #match_arm
            }

            sval::__private::result::Result::Ok(())
        }),
    )
}
