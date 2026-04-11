use syn::{Attribute, Fields, Generics, Ident, Path, Variant};

use crate::{
    attr::{self, SvalAttribute},
    derive::{
        derive_newtype::NewtypeAttrs, derive_struct::StructAttrs,
        derive_unit_struct::UnitStructAttrs, infer_ref_lifetime, ImplStrategy, ImplValue,
        ImplValueRef,
    },
    index::{quote_optional_index, Index, IndexAllocator, IndexValue},
    label::{label_or_ident, quote_optional_label, LabelValue},
    stream::{stream_newtype, stream_record_tuple, stream_tag, RecordTupleTarget},
    tag::{quote_optional_tag, quote_optional_tag_owned},
};

pub(crate) struct EnumAttrs {
    tag: Option<Path>,
    label: Option<LabelValue>,
    index: Option<IndexValue>,
    unlabeled_variants: bool,
    unindexed_variants: bool,
    dynamic: bool,
    ref_attr: Option<crate::attr::RefAttrValue>,
}

impl EnumAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        attr::check(
            "enum",
            &[
                &attr::TagAttr,
                &attr::LabelAttr,
                &attr::IndexAttr,
                &attr::DynamicAttr,
                &attr::UnlabeledVariantsAttr,
                &attr::UnindexedVariantsAttr,
                &attr::RefAttr,
            ],
            attrs,
        )?;

        let tag = attr::get("enum", attr::TagAttr, attrs)?;
        let label = attr::get("enum", attr::LabelAttr, attrs)?;
        let index = attr::get("enum", attr::IndexAttr, attrs)?;
        let unlabeled_variants =
            attr::get("enum", attr::UnlabeledVariantsAttr, attrs)?.unwrap_or(false);
        let unindexed_variants =
            attr::get("enum", attr::UnindexedVariantsAttr, attrs)?.unwrap_or(false);
        let dynamic = attr::get("enum", attr::DynamicAttr, attrs)?.unwrap_or(false);
        let ref_attr = attr::get("enum", attr::RefAttr, attrs)?;

        if dynamic {
            if tag.is_some() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "dynamic enums can't have tags",
                ));
            }
            if label.is_some() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "dynamic enums can't have labels",
                ));
            }
            if index.is_some() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "dynamic enums can't have indexes",
                ));
            }

            if unlabeled_variants {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "dynamic enums don't have variants",
                ));
            }
            if unindexed_variants {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "dynamic enums don't have variants",
                ));
            }
        }

        Ok(EnumAttrs {
            tag,
            label,
            index,
            unlabeled_variants,
            unindexed_variants,
            dynamic,
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

    pub(crate) fn value_ref_lifetime(&self) -> Option<&crate::attr::RefAttrValue> {
        self.ref_attr.as_ref()
    }
}

pub(crate) fn derive_enum<'a>(
    ident: &Ident,
    generics: &Generics,
    variants: impl Iterator<Item = &'a Variant> + 'a,
    attrs: &EnumAttrs,
) -> syn::Result<proc_macro2::TokenStream> {
    let variants: Vec<_> = variants.collect();

    let mut impl_blocks = vec![ImplValue::new(Some(quote_optional_tag_owned(attrs.tag()))).boxed()];

    // Include a derive for `ValueRef` if the type carries a `#[sval(ref)]` attribute
    if let Some(ref_attr) = attrs.value_ref_lifetime() {
        impl_blocks.push(
            ImplValueRef::new(
                match ref_attr.lifetime() {
                    Some(lifetime) => lifetime.clone(),
                    None => infer_ref_lifetime(generics)?,
                },
                vec![], // Empty inner_ref_fields for enums
            )
            .boxed(),
        );
    }

    let mut impl_tokens = Vec::new();

    for block in &impl_blocks {
        let mut variant_match_arms = Vec::new();
        let mut index_allocator = IndexAllocator::new();

        let mut variant_index = |index: Option<Index>, discriminant: Option<IndexValue>| {
            index.or_else(|| {
                if attrs.dynamic || attrs.unindexed_variants {
                    None
                } else {
                    Some(index_allocator.next_const_index(discriminant))
                }
            })
        };

        let variant_label = |label: Option<LabelValue>, ident: &Ident| {
            if attrs.dynamic || attrs.unlabeled_variants {
                None
            } else {
                Some(label_or_ident(label, ident))
            }
        };

        let unit_variant_label = |label: Option<LabelValue>, ident: &Ident| {
            if attrs.unlabeled_variants {
                None
            } else {
                Some(label_or_ident(label, ident))
            }
        };

        let variant_transparent = attrs.dynamic;

        for variant in &variants {
            // Only allow a subset of attributes on enum variants
            // We need to make sure variants are always wrapped in
            // a type that accepts tags, like `tag`, `tagged`, record`,
            // or `tuple`.
            attr::check(
                "enum variant",
                &[
                    &attr::TagAttr,
                    &attr::LabelAttr,
                    &attr::IndexAttr,
                    &attr::UnlabeledFieldsAttr,
                    &attr::UnindexedFieldsAttr,
                ],
                &variant.attrs,
            )?;

            let discriminant = if let Some((_, discriminant)) = &variant.discriminant {
                Some(attr::IndexAttr.from_expr(discriminant)?)
            } else {
                None
            };

            let variant_ident = &variant.ident;

            variant_match_arms.push(match variant.fields {
                Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                    let attrs = NewtypeAttrs::from_attrs(&variant.attrs)?;

                    stream_newtype(
                        quote!(#ident :: #variant_ident),
                        &fields.unnamed[0],
                        &**block,
                        attrs.tag(),
                        variant_label(attrs.label(), variant_ident),
                        variant_index(attrs.index(), discriminant),
                        variant_transparent,
                    )?
                }
                Fields::Unit => {
                    let attrs = UnitStructAttrs::from_attrs(&variant.attrs)?;

                    stream_tag(
                        quote!(#ident :: #variant_ident),
                        attrs.tag(),
                        unit_variant_label(attrs.label(), variant_ident),
                        variant_index(attrs.index(), discriminant),
                    )?
                }
                Fields::Named(ref fields) => {
                    let attrs = StructAttrs::from_attrs(&variant.attrs)?;

                    stream_record_tuple(
                        quote!(#ident :: #variant_ident),
                        fields.named.iter(),
                        &**block,
                        RecordTupleTarget::named_fields(),
                        attrs.tag(),
                        variant_label(attrs.label(), variant_ident),
                        variant_index(attrs.index(), discriminant),
                        attrs.unlabeled_fields(),
                        attrs.unindexed_fields(),
                    )?
                }
                Fields::Unnamed(ref fields) => {
                    let attrs = StructAttrs::from_attrs(&variant.attrs)?;

                    stream_record_tuple(
                        quote!(#ident :: #variant_ident),
                        fields.unnamed.iter(),
                        &**block,
                        RecordTupleTarget::unnamed_fields(),
                        attrs.tag(),
                        variant_label(attrs.label(), variant_ident),
                        variant_index(attrs.index(), discriminant),
                        attrs.unlabeled_fields(),
                        attrs.unindexed_fields(),
                    )?
                }
            });
        }

        let impl_body = if attrs.dynamic {
            quote!({
                match self {
                    #(#variant_match_arms)*
                }

                sval::__private::result::Result::Ok(())
            })
        } else {
            let tag = quote_optional_tag(attrs.tag());
            let label = quote_optional_label(Some(label_or_ident(attrs.label(), ident)));
            let index = quote_optional_index(attrs.index());

            quote!({
                stream.enum_begin(#tag, #label, #index)?;

                match self {
                    #(#variant_match_arms)*
                }

                stream.enum_end(#tag, #label, #index)
            })
        };

        impl_tokens.push(block.quote_impl(ident, generics, impl_body)?);
    }

    Ok(quote!(#(#impl_tokens)*))
}
