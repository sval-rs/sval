use syn::{Attribute, Fields, Generics, Ident, Path, Variant};

use crate::{
    attr::{self, SvalAttribute},
    bound,
    derive::{
        derive_newtype::NewtypeAttrs, derive_struct::StructAttrs,
        derive_unit_struct::UnitStructAttrs, impl_tokens,
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
}

impl EnumAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        attr::check(
            "enum",
            &[
                &attr::TagAttr,
                &attr::LabelAttr,
                &attr::IndexAttr,
                &attr::DynamicAttr,
                &attr::UnlabeledVariantsAttr,
                &attr::UnindexedVariantsAttr,
            ],
            attrs,
        );

        let tag = attr::get_unchecked("enum", attr::TagAttr, attrs);
        let label = attr::get_unchecked("enum", attr::LabelAttr, attrs);
        let index = attr::get_unchecked("enum", attr::IndexAttr, attrs);
        let unlabeled_variants =
            attr::get_unchecked("enum", attr::UnlabeledVariantsAttr, attrs).unwrap_or(false);
        let unindexed_variants =
            attr::get_unchecked("enum", attr::UnindexedVariantsAttr, attrs).unwrap_or(false);
        let dynamic = attr::get_unchecked("enum", attr::DynamicAttr, attrs).unwrap_or(false);

        if dynamic {
            assert!(tag.is_none(), "dynamic enums can't have tags");
            assert!(label.is_none(), "dynamic enums can't have labels");
            assert!(index.is_none(), "dynamic enums can't have indexes");
        }

        EnumAttrs {
            tag,
            label,
            index,
            unlabeled_variants,
            unindexed_variants,
            dynamic,
        }
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

pub(crate) fn derive_enum<'a>(
    ident: &Ident,
    generics: &Generics,
    variants: impl Iterator<Item = &'a Variant> + 'a,
    attrs: &EnumAttrs,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

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

    for variant in variants {
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
        );

        let discriminant = variant
            .discriminant
            .as_ref()
            .and_then(|(_, discriminant)| attr::IndexAttr.try_from_expr(discriminant));

        let variant_ident = &variant.ident;

        variant_match_arms.push(match variant.fields {
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let attrs = NewtypeAttrs::from_attrs(&variant.attrs);

                stream_newtype(
                    quote!(#ident :: #variant_ident),
                    &fields.unnamed[0],
                    attrs.tag(),
                    variant_label(attrs.label(), variant_ident),
                    variant_index(attrs.index(), discriminant),
                    variant_transparent,
                )
            }
            Fields::Unit => {
                let attrs = UnitStructAttrs::from_attrs(&variant.attrs);

                stream_tag(
                    quote!(#ident :: #variant_ident),
                    attrs.tag(),
                    unit_variant_label(attrs.label(), variant_ident),
                    variant_index(attrs.index(), discriminant),
                )
            }
            Fields::Named(ref fields) => {
                let attrs = StructAttrs::from_attrs(&variant.attrs);

                stream_record_tuple(
                    quote!(#ident :: #variant_ident),
                    fields.named.iter(),
                    RecordTupleTarget::named_fields(),
                    attrs.tag(),
                    variant_label(attrs.label(), variant_ident),
                    variant_index(attrs.index(), discriminant),
                    attrs.unlabeled_fields(),
                    attrs.unindexed_fields(),
                )
            }
            Fields::Unnamed(ref fields) => {
                let attrs = StructAttrs::from_attrs(&variant.attrs);

                stream_record_tuple(
                    quote!(#ident :: #variant_ident),
                    fields.unnamed.iter(),
                    RecordTupleTarget::unnamed_fields(),
                    attrs.tag(),
                    variant_label(attrs.label(), variant_ident),
                    variant_index(attrs.index(), discriminant),
                    attrs.unlabeled_fields(),
                    attrs.unindexed_fields(),
                )
            }
        });
    }

    if attrs.dynamic {
        impl_tokens(
            impl_generics,
            ident,
            ty_generics,
            &bounded_where_clause,
            quote!({
                match self {
                    #(#variant_match_arms)*
                }

                Ok(())
            }),
            None,
        )
    } else {
        let tag = quote_optional_tag(attrs.tag());
        let tag_owned = quote_optional_tag_owned(attrs.tag());
        let label = quote_optional_label(Some(label_or_ident(attrs.label(), ident)));
        let index = quote_optional_index(attrs.index());

        impl_tokens(
            impl_generics,
            ident,
            ty_generics,
            &bounded_where_clause,
            quote!({
                stream.enum_begin(#tag, #label, #index)?;

                match self {
                    #(#variant_match_arms)*
                }

                stream.enum_end(#tag, #label, #index)
            }),
            Some(tag_owned),
        )
    }
}
