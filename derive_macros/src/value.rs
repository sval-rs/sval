use std::borrow::Cow;

use crate::{
    attr::{self, SvalAttribute},
    bound,
};
use proc_macro::TokenStream;
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
    FieldsUnnamed, Generics, Ident, Path, Variant,
};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    let tag = attr::container(attr::Tag, &input.attrs);
    let label = attr::container(attr::Label, &input.attrs);
    let index = IndexAllocator::index_of(attr::container(attr::Index, &input.attrs));
    let unlabeled = attr::container(attr::Unlabeled, &input.attrs).unwrap_or(false);
    let unindexed = attr::container(attr::Unindexed, &input.attrs).unwrap_or(false);

    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => derive_unit_struct(
            tag.as_ref(),
            label.as_deref(),
            index,
            &input.ident,
            &input.generics,
        ),
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => derive_struct(
            tag.as_ref(),
            label.as_deref(),
            index,
            unlabeled,
            unindexed,
            &input.ident,
            &input.generics,
            fields,
        ),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) if fields.unnamed.len() == 1 => derive_newtype(
            tag.as_ref(),
            label.as_deref(),
            index,
            &input.ident,
            &input.generics,
            &fields.unnamed[0],
        ),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) => derive_tuple(
            tag.as_ref(),
            label.as_deref(),
            index,
            unlabeled,
            unindexed,
            &input.ident,
            &input.generics,
            fields,
        ),
        Data::Enum(DataEnum { ref variants, .. }) if variants.len() == 0 => {
            derive_void(&input.ident, &input.generics)
        }
        Data::Enum(DataEnum { variants, .. }) => derive_enum(
            tag.as_ref(),
            label.as_deref(),
            index,
            &input.ident,
            &input.generics,
            variants.iter(),
        ),
        _ => panic!("unimplemented"),
    }
}

fn derive_struct<'a>(
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
    unlabeled: bool,
    unindexed: bool,
    ident: &Ident,
    generics: &Generics,
    fields: &FieldsNamed,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let label = label_or_ident(label, ident);

    let match_arm = stream_record_tuple(
        quote!(#ident),
        tag,
        &label,
        index,
        fields.named.iter(),
        RecordTupleTarget::named_fields(),
        unlabeled,
        unindexed,
    );

    let tag = quote_optional_tag_owned(tag);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    match self {
                        #match_arm
                    }

                    Ok(())
                }

                fn tag(&self) -> Option<sval::Tag> {
                    #tag
                }
            }
        };
    })
}

fn derive_unit_struct<'a>(
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
    ident: &Ident,
    generics: &Generics,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let label = label_or_ident(label, ident);

    let match_arm = stream_tag(quote!(_), tag, &label, index);

    let tag = quote_optional_tag_owned(tag);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    match self {
                        #match_arm
                    }

                    Ok(())
                }

                fn tag(&self) -> Option<sval::Tag> {
                    #tag
                }
            }
        };
    })
}

fn derive_newtype<'a>(
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
    ident: &Ident,
    generics: &Generics,
    field: &Field,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    attr::ensure_newtype_field_empty(&field.attrs);

    let label = label_or_ident(label, ident);

    let match_arm = stream_newtype(quote!(#ident), tag, &label, index);

    let tag = quote_optional_tag_owned(tag);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    match self {
                        #match_arm
                    }

                    Ok(())
                }

                fn tag(&self) -> Option<sval::Tag> {
                    #tag
                }
            }
        };
    })
}

fn derive_tuple<'a>(
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
    unlabeled: bool,
    unindexed: bool,
    ident: &Ident,
    generics: &Generics,
    fields: &FieldsUnnamed,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let label = label_or_ident(label, ident);

    let match_arm = stream_record_tuple(
        quote!(#ident),
        tag,
        &label,
        index,
        fields.unnamed.iter(),
        RecordTupleTarget::unnamed_fields(),
        unlabeled,
        unindexed,
    );

    let tag = quote_optional_tag_owned(tag);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    match self {
                        #match_arm
                    }

                    Ok(())
                }

                fn tag(&self) -> Option<sval::Tag> {
                    #tag
                }
            }
        };
    })
}

fn derive_enum<'a>(
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
    ident: &Ident,
    generics: &Generics,
    variants: impl Iterator<Item = &'a Variant> + 'a,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let label = label_or_ident(label, ident);

    let enum_tag = quote_optional_tag(tag);
    let enum_label = quote_optional_label(Some(&label));
    let enum_index = quote_optional_index(index);

    let mut variant_match_arms = Vec::new();
    let mut index_allocator = IndexAllocator::new();

    for variant in variants {
        let tag = attr::container(attr::Tag, &variant.attrs);
        let label = attr::container(attr::Label, &variant.attrs)
            .unwrap_or_else(|| variant.ident.to_string());
        let unlabeled = attr::container(attr::Unlabeled, &variant.attrs).unwrap_or(false);
        let unindexed = attr::container(attr::Unindexed, &variant.attrs).unwrap_or(false);

        // If there's a discriminant, use it as the index
        let index = index_allocator.next_index(
            attr::container(attr::Index, &variant.attrs).or_else(|| {
                variant
                    .discriminant
                    .as_ref()
                    .and_then(|(_, discriminant)| attr::Index.from_expr(discriminant))
            }),
        );

        let variant_ident = &variant.ident;

        variant_match_arms.push(match variant.fields {
            Fields::Named(ref fields) => stream_record_tuple(
                quote!(#ident :: #variant_ident),
                tag.as_ref(),
                &label,
                Some(index),
                fields.named.iter(),
                RecordTupleTarget::named_fields(),
                unlabeled,
                unindexed,
            ),
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => stream_newtype(
                quote!(#ident :: #variant_ident),
                tag.as_ref(),
                &label,
                Some(index),
            ),
            Fields::Unnamed(ref fields) => stream_record_tuple(
                quote!(#ident :: #variant_ident),
                tag.as_ref(),
                &label,
                Some(index),
                fields.unnamed.iter(),
                RecordTupleTarget::unnamed_fields(),
                unlabeled,
                unindexed,
            ),
            Fields::Unit => stream_tag(
                quote!(#ident :: #variant_ident),
                tag.as_ref(),
                &label,
                Some(index),
            ),
        });
    }

    let tag = quote_optional_tag_owned(tag);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    stream.enum_begin(#enum_tag, #enum_label, #enum_index)?;

                    match self {
                        #(#variant_match_arms)*
                    }

                    stream.enum_end(#enum_tag, #enum_label, #enum_index)
                }

                fn tag(&self) -> Option<sval::Tag> {
                    #tag
                }
            }
        };
    })
}

fn derive_void<'a>(ident: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    match *self {}
                }
            }
        };
    })
}

enum RecordTupleTarget {
    RecordTuple,
    Record,
    Tuple,
    Seq,
}

impl RecordTupleTarget {
    fn named_fields() -> Self {
        RecordTupleTarget::RecordTuple
    }

    fn unnamed_fields() -> Self {
        RecordTupleTarget::Tuple
    }
}

fn stream_record_tuple<'a>(
    path: proc_macro2::TokenStream,
    tag: Option<&Path>,
    label: &str,
    index: Option<Index>,
    fields: impl Iterator<Item = &'a Field>,
    mut target: RecordTupleTarget,
    unlabeled: bool,
    unindexed: bool,
) -> proc_macro2::TokenStream {
    let tag = quote_optional_tag(tag);
    let label = quote_optional_label(Some(label));
    let index = quote_optional_index(index);

    let mut field_binding = Vec::new();
    let mut stream_field = Vec::new();

    let mut field_count = 0usize;
    let mut labeled_field_count = 0;
    let mut indexed_field_count = 0;

    let mut index_allocator = IndexAllocator::new();

    for (i, field) in fields.enumerate() {
        let i = syn::Index::from(i);

        if attr::field(attr::Skip, &field.attrs).unwrap_or(false) {
            field_binding.push(quote_field_skip(&i, field));
            continue;
        }

        let (ident, binding) = get_field(&i, field);

        let tag = quote_optional_tag(attr::field(attr::Tag, &field.attrs).as_ref());

        let label = if unlabeled {
            None
        } else {
            get_label(attr::field(attr::Label, &field.attrs), field.ident.as_ref())
        };

        let index = if unindexed {
            None
        } else {
            Some(quote_index(
                index_allocator.next_index(attr::field(attr::Index, &field.attrs)),
            ))
        };

        match (&label, &index) {
            (Some(label), Some(index)) => {
                stream_field.push(quote!({
                    stream.record_tuple_value_begin(#tag, #label, #index)?;
                    stream.value(#ident)?;
                    stream.record_tuple_value_end(#tag, #label, #index)?;
                }));

                target = RecordTupleTarget::RecordTuple;
                labeled_field_count += 1;
                indexed_field_count += 1;
            }
            (None, Some(index)) => {
                stream_field.push(quote!({
                    stream.tuple_value_begin(#tag, #index)?;
                    stream.value(#ident)?;
                    stream.tuple_value_end(#tag, #index)?;
                }));

                target = RecordTupleTarget::Tuple;
                indexed_field_count += 1;
            }
            (Some(label), None) => {
                stream_field.push(quote!({
                    stream.record_value_begin(#tag, #label, #index)?;
                    stream.value(#ident)?;
                    stream.record_value_end(#tag, #label, #index)?;
                }));

                target = RecordTupleTarget::Record;
                labeled_field_count += 1;
            }
            (None, None) => {
                stream_field.push(quote!({
                    stream.seq_value_begin()?;
                    stream.value(#ident)?;
                    stream.seq_value_end()?;
                }));

                target = RecordTupleTarget::Seq;
            }
        }

        field_binding.push(binding);
        field_count += 1;
    }

    assert!(
        labeled_field_count == 0 || labeled_field_count == field_count,
        "if any fields have a label then all fields need one"
    );
    assert!(
        indexed_field_count == 0 || indexed_field_count == field_count,
        "if any fields have an index then all fields need one"
    );

    match target {
        RecordTupleTarget::RecordTuple => {
            quote!(#path { #(#field_binding,)* } => {
                stream.record_tuple_begin(#tag, #label, #index, Some(#field_count))?;

                #(
                    #stream_field
                )*

                stream.record_tuple_end(#tag, #label, #index)?;
            })
        }
        RecordTupleTarget::Tuple => {
            quote!(#path { #(#field_binding,)* } => {
                stream.tuple_begin(#tag, #label, #index, Some(#field_count))?;

                #(
                    #stream_field
                )*

                stream.tuple_end(#tag, #label, #index)?;
            })
        }
        RecordTupleTarget::Record => {
            quote!(#path { #(#field_binding,)* } => {
                stream.record_begin(#tag, #label, #index, Some(#field_count))?;

                #(
                    #stream_field
                )*

                stream.record_end(#tag, #label, #index)?;
            })
        }
        RecordTupleTarget::Seq => {
            quote!(#path { #(#field_binding,)* } => {
                stream.tagged_begin(#tag, #label, #index)?;
                stream.seq_begin(Some(#field_count))?;

                #(
                    #stream_field
                )*

                stream.seq_end()?;
                stream.tagged_end(#tag, #label, #index)?;
            })
        }
    }
}

fn stream_newtype(
    path: proc_macro2::TokenStream,
    tag: Option<&Path>,
    label: &str,
    index: Option<Index>,
) -> proc_macro2::TokenStream {
    let tag = quote_optional_tag(tag);
    let label = quote_optional_label(Some(label));
    let index = quote_optional_index(index);

    quote!(#path(ref field0) => {
        stream.tagged_begin(#tag, #label, #index)?;
        stream.value(field0)?;
        stream.tagged_end(#tag, #label, #index)?;
    })
}

fn stream_tag(
    path: proc_macro2::TokenStream,
    tag: Option<&Path>,
    label: &str,
    index: Option<Index>,
) -> proc_macro2::TokenStream {
    let tag = quote_optional_tag(tag);
    let label = quote_optional_label(Some(label));
    let index = quote_optional_index(index);

    quote!(#path => {
        stream.tag(#tag, #label, #index)?;
    })
}

fn quote_optional_tag(tag: Option<&Path>) -> proc_macro2::TokenStream {
    match tag {
        Some(tag) => quote!(Some(&#tag)),
        None => quote!(None),
    }
}

fn quote_optional_tag_owned(tag: Option<&Path>) -> proc_macro2::TokenStream {
    match tag {
        Some(tag) => quote!(Some(#tag)),
        None => quote!(None),
    }
}

fn label_or_ident<'a>(label: Option<&'a str>, ident: &'_ Ident) -> Cow<'a, str> {
    label
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(ident.to_string()))
}

fn quote_label(label: &str) -> proc_macro2::TokenStream {
    quote!(&sval::Label::new(#label))
}

fn quote_optional_label(label: Option<&str>) -> proc_macro2::TokenStream {
    match label {
        Some(label) => {
            let label = quote_label(label);
            quote!(Some(#label))
        }
        None => quote!(None),
    }
}

fn quote_index(index: Index) -> proc_macro2::TokenStream {
    match index {
        Index::Explicit(index) => quote!(&sval::Index::from(#index)),
        Index::Implicit(index) => {
            quote!(&sval::Index::from(#index).with_tag(&sval::tags::VALUE_OFFSET))
        }
    }
}

fn quote_optional_index(index: Option<Index>) -> proc_macro2::TokenStream {
    match index {
        Some(index) => {
            let index = quote_index(index);
            quote!(Some(#index))
        }
        None => quote!(None),
    }
}

fn get_field(index: &syn::Index, field: &Field) -> (Ident, proc_macro2::TokenStream) {
    let ident = Ident::new(&format!("field{}", index.index), field.span());

    if let Some(ref field) = field.ident {
        let tokens = quote!(#field: ref #ident);

        (ident, tokens)
    } else {
        let tokens = quote!(#index: ref #ident);

        (ident, tokens)
    }
}

fn get_label(explicit: Option<String>, ident: Option<&Ident>) -> Option<proc_macro2::TokenStream> {
    explicit
        .or_else(|| ident.map(|ident| ident.to_string()))
        .map(|label| quote_label(&label))
}

fn quote_field_skip(index: &syn::Index, field: &Field) -> proc_macro2::TokenStream {
    if let Some(ref field) = field.ident {
        quote!(#field: _)
    } else {
        quote!(#index: _)
    }
}

struct IndexAllocator {
    next_index: isize,
    explicit: bool,
}

impl IndexAllocator {
    fn new() -> Self {
        IndexAllocator {
            next_index: 0,
            explicit: false,
        }
    }

    fn index_of(explicit: Option<isize>) -> Option<Index> {
        explicit.map(Index::Explicit)
    }

    fn next_index(&mut self, explicit: Option<isize>) -> Index {
        if let Some(index) = explicit {
            self.explicit = true;
            self.next_index = index + 1;

            Index::Explicit(index)
        } else {
            let index = self.next_index;
            self.next_index += 1;

            if self.explicit {
                Index::Explicit(index)
            } else {
                Index::Implicit(index)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Index {
    Implicit(isize),
    Explicit(isize),
}
