use syn::{spanned::Spanned, Field, Ident, Path};

use crate::{
    attr::{self},
    index::{quote_index, quote_optional_index, Index, IndexAllocator},
    label::{quote_label, quote_optional_label},
    tag::quote_optional_tag,
};

pub(crate) enum RecordTupleTarget {
    RecordTuple,
    Record,
    Tuple,
    Seq,
}

impl RecordTupleTarget {
    pub(crate) fn named_fields() -> Self {
        RecordTupleTarget::RecordTuple
    }

    pub(crate) fn unnamed_fields() -> Self {
        RecordTupleTarget::Tuple
    }
}

pub(crate) fn stream_record_tuple<'a>(
    path: proc_macro2::TokenStream,
    fields: impl Iterator<Item = &'a Field>,
    mut target: RecordTupleTarget,
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
    unlabeled_fields: bool,
    unindexed_fields: bool,
) -> proc_macro2::TokenStream {
    let tag = quote_optional_tag(tag);
    let label = quote_optional_label(label);
    let index = quote_optional_index(index);

    let mut field_binding = Vec::new();
    let mut stream_field = Vec::new();

    let mut field_count = 0usize;
    let mut labeled_field_count = 0;
    let mut indexed_field_count = 0;
    let mut const_size = true;

    let index_ident = Ident::new("__sval_index", proc_macro2::Span::call_site());
    let label_ident = Ident::new("__sval_label", proc_macro2::Span::call_site());

    let mut index_allocator = IndexAllocator::new();

    for (i, field) in fields.enumerate() {
        attr::check(
            "struct field",
            &[
                &attr::TagAttr,
                &attr::IndexAttr,
                &attr::LabelAttr,
                &attr::SkipAttr,
                &attr::FlattenAttr,
            ],
            &field.attrs,
        );

        let i = syn::Index::from(i);

        if attr::get_unchecked("struct field", attr::SkipAttr, &field.attrs).unwrap_or(false) {
            field_binding.push(quote_field_skip(&i, field));
            continue;
        }

        let (ident, binding) = get_field(&i, field);

        let tag = quote_optional_tag(
            attr::get_unchecked("struct field", attr::TagAttr, &field.attrs).as_ref(),
        );

        let label = if unlabeled_fields {
            None
        } else {
            get_label(
                attr::get_unchecked("struct field", attr::LabelAttr, &field.attrs),
                field.ident.as_ref(),
            )
        };

        let index = if unindexed_fields {
            None
        } else {
            Some(quote_index(index_allocator.next_computed_index(
                &index_ident,
                attr::get_unchecked("struct field", attr::IndexAttr, &field.attrs),
            )))
        };

        let flatten =
            attr::get_unchecked("struct field", attr::FlattenAttr, &field.attrs).unwrap_or(false);

        const_size = const_size && !flatten;

        match (&label, &index) {
            (Some(label), Some(index)) => {
                if flatten {
                    stream_field.push(quote!(#index_ident = sval_derive::extensions::flatten::flatten_to_record_tuple(&mut *stream, #ident, #index_ident)?;));
                } else {
                    stream_field.push(quote!({
                        let #index_ident = #index;
                        let #label_ident = #label;

                        stream.record_tuple_value_begin(#tag, #label_ident, #index_ident)?;
                        stream.value(#ident)?;
                        stream.record_tuple_value_end(#tag, #label_ident, #index_ident)?;
                    }));
                }

                target = RecordTupleTarget::RecordTuple;
                labeled_field_count += 1;
                indexed_field_count += 1;
            }
            (None, Some(index)) => {
                if flatten {
                    stream_field.push(quote!(#index_ident = sval_derive::extensions::flatten::flatten_to_tuple(&mut *stream, #ident, #index_ident)?;));
                } else {
                    stream_field.push(quote!({
                        let #index_ident = #index;

                        stream.tuple_value_begin(#tag, #index_ident)?;
                        stream.value(#ident)?;
                        stream.tuple_value_end(#tag, #index_ident)?;
                    }));
                }

                target = RecordTupleTarget::Tuple;
                indexed_field_count += 1;
            }
            (Some(label), None) => {
                if flatten {
                    stream_field.push(quote!(#index_ident = sval_derive::extensions::flatten::flatten_to_record(&mut *stream, #ident, #index_ident)?;));
                } else {
                    stream_field.push(quote!({
                        let #label_ident = #label;

                        stream.record_value_begin(#tag, #label_ident)?;
                        stream.value(#ident)?;
                        stream.record_value_end(#tag, #label_ident)?;
                    }));
                }

                target = RecordTupleTarget::Record;
                labeled_field_count += 1;
            }
            (None, None) => {
                if flatten {
                    stream_field.push(quote!(sval_derive::extensions::flatten::flatten_to_seq(&mut *stream, #ident)?;));
                } else {
                    stream_field.push(quote!({
                        stream.seq_value_begin()?;
                        stream.value(#ident)?;
                        stream.seq_value_end()?;
                    }));
                }

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

    let field_count = if const_size {
        quote!(Some(#field_count))
    } else {
        quote!(None)
    };

    match target {
        RecordTupleTarget::RecordTuple => {
            quote!(#path { #(#field_binding,)* } => {
                stream.record_tuple_begin(#tag, #label, #index, #field_count)?;

                let mut #index_ident = 0;

                #(
                    #stream_field
                )*

                stream.record_tuple_end(#tag, #label, #index)?;
            })
        }
        RecordTupleTarget::Tuple => {
            quote!(#path { #(#field_binding,)* } => {
                stream.tuple_begin(#tag, #label, #index, #field_count)?;

                let mut #index_ident = 0;

                #(
                    #stream_field
                )*

                stream.tuple_end(#tag, #label, #index)?;
            })
        }
        RecordTupleTarget::Record => {
            quote!(#path { #(#field_binding,)* } => {
                stream.record_begin(#tag, #label, #index, #field_count)?;

                let mut #index_ident = 0;

                #(
                    #stream_field
                )*

                stream.record_end(#tag, #label, #index)?;
            })
        }
        RecordTupleTarget::Seq => {
            quote!(#path { #(#field_binding,)* } => {
                stream.tagged_begin(#tag, #label, #index)?;
                stream.seq_begin(#field_count)?;

                #(
                    #stream_field
                )*

                stream.seq_end()?;
                stream.tagged_end(#tag, #label, #index)?;
            })
        }
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
