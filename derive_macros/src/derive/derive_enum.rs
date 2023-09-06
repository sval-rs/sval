use std::borrow::Cow;

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

/**
Get an attribute that is applicable to a struct.
*/
fn enum_container<T: SvalAttribute>(request: T, attrs: &[Attribute]) -> Option<T::Result> {
    attr::get(
        "enum",
        &[
            &attr::TagAttr,
            &attr::LabelAttr,
            &attr::IndexAttr,
            &attr::DynamicAttr,
        ],
        request,
        attrs,
    )
}

pub(crate) struct EnumAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
    dynamic: bool,
}

impl EnumAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        let tag = enum_container(attr::TagAttr, attrs);
        let label = enum_container(attr::LabelAttr, attrs);
        let index = enum_container(attr::IndexAttr, attrs);
        let dynamic = enum_container(attr::DynamicAttr, attrs).unwrap_or(false);

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

    pub(crate) fn label(&self, ident: &Ident) -> Cow<str> {
        label_or_ident(self.label.as_deref(), ident)
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

    for variant in variants {
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
                    Some(&*attrs.label(variant_ident)),
                    variant_index(attrs.index(), discriminant),
                )
            }
            Fields::Unit => {
                let attrs = UnitStructAttrs::from_attrs(&variant.attrs);

                stream_tag(
                    quote!(#ident :: #variant_ident),
                    attrs.tag(),
                    Some(&*attrs.label(variant_ident)),
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
                    Some(&*attrs.label(variant_ident)),
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
                    Some(&*attrs.label(variant_ident)),
                    variant_index(attrs.index(), discriminant),
                    attrs.unlabeled_fields(),
                    attrs.unindexed_fields(),
                )
            }
        });
    }

    if attrs.dynamic {
        assert!(attrs.tag.is_none(), "dynamic enums can't have tags");
        assert!(attrs.label.is_none(), "dynamic enums can't have labels");
        assert!(attrs.index.is_none(), "dynamic enums can't have indexes");

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
        let label = quote_optional_label(Some(&*attrs.label(ident)));
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