use syn::{Attribute, Fields, Generics, Ident, Path};

use crate::{
    attr,
    attr::RefAttrValue,
    derive::{
        collect_inner_ref_field_types, infer_ref_lifetime, ImplStrategy, ImplValue, ImplValueRef,
    },
    index::{Index, IndexAllocator, IndexValue},
    label::{label_or_ident, LabelValue},
    stream::{stream_record_tuple, RecordTupleTarget},
    tag::quote_optional_tag_owned,
};

pub(crate) struct StructAttrs {
    tag: Option<Path>,
    label: Option<LabelValue>,
    index: Option<IndexValue>,
    unlabeled_fields: bool,
    unindexed_fields: bool,
    ref_attr: Option<RefAttrValue>,
}

impl StructAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        attr::check(
            "struct",
            &[
                &attr::TagAttr,
                &attr::LabelAttr,
                &attr::IndexAttr,
                &attr::UnlabeledFieldsAttr,
                &attr::UnindexedFieldsAttr,
                &attr::RefAttr,
            ],
            attrs,
        )?;

        let tag = attr::get("struct", attr::TagAttr, attrs)?;
        let label = attr::get("struct", attr::LabelAttr, attrs)?;
        let index = attr::get("struct", attr::IndexAttr, attrs)?;

        let unlabeled_fields =
            attr::get("struct", attr::UnlabeledFieldsAttr, attrs)?.unwrap_or(false);
        let unindexed_fields =
            attr::get("struct", attr::UnindexedFieldsAttr, attrs)?.unwrap_or(false);
        let ref_attr = attr::get("struct", attr::RefAttr, attrs)?;

        Ok(StructAttrs {
            tag,
            label,
            index,
            unlabeled_fields,
            unindexed_fields,
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

    pub(crate) fn unlabeled_fields(&self) -> bool {
        self.unlabeled_fields
    }

    pub(crate) fn unindexed_fields(&self) -> bool {
        self.unindexed_fields
    }

    pub(crate) fn value_ref_lifetime(&self) -> Option<&RefAttrValue> {
        self.ref_attr.as_ref()
    }
}

pub(crate) fn derive_struct<'a>(
    ident: &Ident,
    generics: &Generics,
    fields: &Fields,
    attrs: &StructAttrs,
) -> syn::Result<proc_macro2::TokenStream> {
    let (fields, target) = match fields {
        Fields::Named(ref fields) => (&fields.named, RecordTupleTarget::named_fields()),
        Fields::Unnamed(ref fields) => (&fields.unnamed, RecordTupleTarget::unnamed_fields()),
        Fields::Unit => {
            // Unreachable: unit structs are handled separately in derive()
            unreachable!()
        }
    };

    let mut impl_blocks = vec![ImplValue::new(Some(quote_optional_tag_owned(attrs.tag()))).boxed()];

    // Include a derive for `ValueRef` if the type carries a `#[sval(ref)]` attribute
    if let Some(lf) = attrs.value_ref_lifetime() {
        impl_blocks.push(
            ImplValueRef::new(
                match lf.lifetime() {
                    Some(lf) => lf.clone(),
                    None => infer_ref_lifetime(generics)?,
                },
                collect_inner_ref_field_types(fields.iter())?,
            )
            .boxed(),
        );
    };

    let mut impl_tokens = Vec::new();

    for block in impl_blocks {
        let match_arm = stream_record_tuple(
            quote!(#ident),
            fields.iter(),
            &*block,
            target,
            attrs.tag(),
            Some(label_or_ident(attrs.label(), ident)),
            attrs.index(),
            attrs.unlabeled_fields(),
            attrs.unindexed_fields(),
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
