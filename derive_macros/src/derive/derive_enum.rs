use syn::{Attribute, Fields, Generics, Ident, Path, Variant};

use crate::{
    attr::{self, SvalAttribute},
    bound,
    derive::{
        derive_newtype::NewtypeAttrs, derive_struct::StructAttrs,
        derive_unit_struct::UnitStructAttrs,
    },
    index::{quote_optional_index, Index, IndexAllocator},
    label::{label_or_ident, quote_optional_label},
    stream::{stream_newtype, stream_record_tuple, stream_tag, RecordTupleTarget},
    tag::{quote_optional_tag, quote_optional_tag_owned},
};

pub(crate) struct EnumAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
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
            ],
            attrs,
        );

        let tag = attr::get_unchecked("enum", attr::TagAttr, attrs);
        let label = attr::get_unchecked("enum", attr::LabelAttr, attrs);
        let index = attr::get_unchecked("enum", attr::IndexAttr, attrs);
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
            dynamic,
        }
    }

    pub(crate) fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    pub(crate) fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub(crate) fn index(&self) -> Option<Index> {
        self.index.map(IndexAllocator::const_index_of)
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

    let mut variant_index = |index: Option<Index>, discriminant: Option<isize>| {
        index.or_else(|| {
            if attrs.dynamic {
                None
            } else {
                Some(index_allocator.next_const_index(discriminant))
            }
        })
    };

    let variant_label = |label: Option<&str>, ident: &Ident| {
        if attrs.dynamic {
            None
        } else {
            Some(label_or_ident(label, ident).into_owned())
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
            .and_then(|(_, discriminant)| attr::IndexAttr.from_expr(discriminant));

        let variant_ident = &variant.ident;

        variant_match_arms.push(match variant.fields {
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let attrs = NewtypeAttrs::from_attrs(&variant.attrs);

                stream_newtype(
                    quote!(#ident :: #variant_ident),
                    &fields.unnamed[0],
                    attrs.tag(),
                    variant_label(attrs.label(), variant_ident).as_deref(),
                    variant_index(attrs.index(), discriminant),
                    variant_transparent,
                )
            }
            Fields::Unit => {
                let attrs = UnitStructAttrs::from_attrs(&variant.attrs);

                stream_tag(
                    quote!(#ident :: #variant_ident),
                    attrs.tag(),
                    Some(&*label_or_ident(attrs.label(), variant_ident)),
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
                    variant_label(attrs.label(), variant_ident).as_deref(),
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
                    variant_label(attrs.label(), variant_ident).as_deref(),
                    variant_index(attrs.index(), discriminant),
                    attrs.unlabeled_fields(),
                    attrs.unindexed_fields(),
                )
            }
        });
    }

    if attrs.dynamic {
        quote! {
            const _: () = {
                extern crate sval;

                impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                        match self {
                            #(#variant_match_arms)*
                        }

                        Ok(())
                    }
                }
            };
        }
    } else {
        let tag = quote_optional_tag(attrs.tag());
        let tag_owned = quote_optional_tag_owned(attrs.tag());
        let label = quote_optional_label(Some(&*label_or_ident(attrs.label(), ident)));
        let index = quote_optional_index(attrs.index());

        quote! {
            const _: () = {
                extern crate sval;

                impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                        stream.enum_begin(#tag, #label, #index)?;

                        match self {
                            #(#variant_match_arms)*
                        }

                        stream.enum_end(#tag, #label, #index)
                    }

                    fn tag(&self) -> Option<sval::Tag> {
                        #tag_owned
                    }
                }
            };
        }
    }
}
