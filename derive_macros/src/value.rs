use std::borrow::Cow;

use crate::{
    attr::{self, SvalAttribute},
    bound,
};
use proc_macro::TokenStream;
use syn::{
    spanned::Spanned, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Generics,
    Ident, Path, Variant,
};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            let attrs = UnitAttrs::from_attrs(&input.attrs);

            derive_unit_struct(&input.ident, &input.generics, &attrs)
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) if fields.unnamed.len() == 1 => {
            let attrs = NewtypeAttrs::from_attrs(&input.attrs);

            derive_newtype(&input.ident, &input.generics, &fields.unnamed[0], &attrs)
        }
        Data::Struct(DataStruct { ref fields, .. }) => {
            let attrs = StructAttrs::from_attrs(&input.attrs);

            derive_struct(&input.ident, &input.generics, fields, &attrs)
        }
        Data::Enum(DataEnum { ref variants, .. }) if variants.len() == 0 => {
            let attrs = VoidAttrs::from_attrs(&input.attrs);

            derive_void(&input.ident, &input.generics, &attrs)
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let attrs = EnumAttrs::from_attrs(&input.attrs);

            derive_enum(&input.ident, &input.generics, variants.iter(), &attrs)
        }
        _ => panic!("unimplemented"),
    }
}

struct StructAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
    unlabeled_fields: bool,
    unindexed_fields: bool,
}

impl StructAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Self {
        let tag = attr::struct_container(attr::Tag, attrs);
        let label = attr::struct_container(attr::Label, attrs);
        let index = attr::struct_container(attr::Index, attrs);

        let unlabeled_fields = attr::struct_container(attr::Unlabeled, attrs).unwrap_or(false);
        let unindexed_fields = attr::struct_container(attr::Unindexed, attrs).unwrap_or(false);

        StructAttrs {
            tag,
            label,
            index,
            unlabeled_fields,
            unindexed_fields,
        }
    }

    fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    fn label(&self, ident: &Ident) -> Cow<str> {
        label_or_ident(self.label.as_deref(), ident)
    }

    fn index(&self) -> Option<Index> {
        self.index.map(IndexAllocator::const_index_of)
    }

    fn unlabeled_fields(&self) -> bool {
        self.unlabeled_fields
    }

    fn unindexed_fields(&self) -> bool {
        self.unindexed_fields
    }
}

fn derive_struct<'a>(
    ident: &Ident,
    generics: &Generics,
    fields: &Fields,
    attrs: &StructAttrs,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let (fields, target) = match fields {
        Fields::Named(ref fields) => (&fields.named, RecordTupleTarget::named_fields()),
        Fields::Unnamed(ref fields) => (&fields.unnamed, RecordTupleTarget::unnamed_fields()),
        _ => unreachable!(),
    };

    let match_arm = stream_record_tuple(
        quote!(#ident),
        fields.iter(),
        target,
        attrs.tag(),
        Some(&*attrs.label(ident)),
        attrs.index(),
        attrs.unlabeled_fields(),
        attrs.unindexed_fields(),
    );

    let tag = quote_optional_tag_owned(attrs.tag());

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

struct UnitAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
}

impl UnitAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Self {
        let tag = attr::unit_container(attr::Tag, attrs);
        let label = attr::unit_container(attr::Label, attrs);
        let index = attr::unit_container(attr::Index, attrs);

        UnitAttrs { tag, label, index }
    }

    fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    fn label(&self, ident: &Ident) -> Cow<str> {
        label_or_ident(self.label.as_deref(), ident)
    }

    fn index(&self) -> Option<Index> {
        self.index.map(IndexAllocator::const_index_of)
    }
}

fn derive_unit_struct<'a>(ident: &Ident, generics: &Generics, attrs: &UnitAttrs) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_tag(
        quote!(_),
        attrs.tag(),
        Some(&*attrs.label(ident)),
        attrs.index(),
    );

    let tag = quote_optional_tag_owned(attrs.tag());

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

struct NewtypeAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
}

impl NewtypeAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Self {
        let tag = attr::newtype_container(attr::Tag, attrs);
        let label = attr::newtype_container(attr::Label, attrs);
        let index = attr::newtype_container(attr::Index, attrs);

        NewtypeAttrs { tag, label, index }
    }

    fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    fn label(&self, ident: &Ident) -> Cow<str> {
        label_or_ident(self.label.as_deref(), ident)
    }

    fn index(&self) -> Option<Index> {
        self.index.map(IndexAllocator::const_index_of)
    }
}

fn derive_newtype<'a>(
    ident: &Ident,
    generics: &Generics,
    field: &Field,
    attrs: &NewtypeAttrs,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    attr::ensure_newtype_field_empty(&field.attrs);

    let match_arm = stream_newtype(
        quote!(#ident),
        attrs.tag(),
        Some(&*attrs.label(ident)),
        attrs.index(),
    );

    let tag = quote_optional_tag_owned(attrs.tag());

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

struct EnumAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
}

impl EnumAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Self {
        let tag = attr::enum_container(attr::Tag, attrs);
        let label = attr::enum_container(attr::Label, attrs);
        let index = attr::enum_container(attr::Index, attrs);

        EnumAttrs { tag, label, index }
    }

    fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    fn label(&self, ident: &Ident) -> Cow<str> {
        label_or_ident(self.label.as_deref(), ident)
    }

    fn index(&self) -> Option<Index> {
        self.index.map(IndexAllocator::const_index_of)
    }
}

fn derive_enum<'a>(
    ident: &Ident,
    generics: &Generics,
    variants: impl Iterator<Item = &'a Variant> + 'a,
    attrs: &EnumAttrs,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let mut variant_match_arms = Vec::new();
    let mut index_allocator = IndexAllocator::new();

    for variant in variants {
        let discriminant = variant
            .discriminant
            .as_ref()
            .and_then(|(_, discriminant)| attr::Index.from_expr(discriminant));

        let variant_ident = &variant.ident;

        variant_match_arms.push(match variant.fields {
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let attrs = NewtypeAttrs::from_attrs(&variant.attrs);

                stream_newtype(
                    quote!(#ident :: #variant_ident),
                    attrs.tag(),
                    Some(&*attrs.label(variant_ident)),
                    Some(
                        attrs
                            .index()
                            .unwrap_or_else(|| index_allocator.next_const_index(discriminant)),
                    ),
                )
            }
            Fields::Unit => {
                let attrs = UnitAttrs::from_attrs(&variant.attrs);

                stream_tag(
                    quote!(#ident :: #variant_ident),
                    attrs.tag(),
                    Some(&*attrs.label(variant_ident)),
                    Some(
                        attrs
                            .index()
                            .unwrap_or_else(|| index_allocator.next_const_index(discriminant)),
                    ),
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
                    Some(
                        attrs
                            .index()
                            .unwrap_or_else(|| index_allocator.next_const_index(discriminant)),
                    ),
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
                    Some(
                        attrs
                            .index()
                            .unwrap_or_else(|| index_allocator.next_const_index(discriminant)),
                    ),
                    attrs.unlabeled_fields(),
                    attrs.unindexed_fields(),
                )
            }
        });
    }

    let tag = quote_optional_tag(attrs.tag());
    let tag_owned = quote_optional_tag_owned(attrs.tag());
    let label = quote_optional_label(Some(&*attrs.label(ident)));
    let index = quote_optional_index(attrs.index());

    TokenStream::from(quote! {
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
    })
}

struct VoidAttrs {}

impl VoidAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Self {
        attr::ensure_void_empty(attrs);

        VoidAttrs {}
    }
}

fn derive_void<'a>(ident: &Ident, generics: &Generics, attrs: &VoidAttrs) -> TokenStream {
    let _ = attrs;

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

    let index_ident = Ident::new("__sval_index", proc_macro2::Span::call_site());
    let label_ident = Ident::new("__sval_label", proc_macro2::Span::call_site());

    let mut index_allocator = IndexAllocator::new();

    for (i, field) in fields.enumerate() {
        let i = syn::Index::from(i);

        if attr::struct_field(attr::Skip, &field.attrs).unwrap_or(false) {
            field_binding.push(quote_field_skip(&i, field));
            continue;
        }

        let (ident, binding) = get_field(&i, field);

        let tag = quote_optional_tag(attr::struct_field(attr::Tag, &field.attrs).as_ref());

        let label = if unlabeled_fields {
            None
        } else {
            get_label(
                attr::struct_field(attr::Label, &field.attrs),
                field.ident.as_ref(),
            )
        };

        let index = if unindexed_fields {
            None
        } else {
            Some(quote_index(index_allocator.next_computed_index(
                &index_ident,
                attr::struct_field(attr::Index, &field.attrs),
            )))
        };

        match (&label, &index) {
            (Some(label), Some(index)) => {
                stream_field.push(quote!({
                    let #index_ident = #index;
                    let #label_ident = #label;

                    stream.record_tuple_value_begin(#tag, #label_ident, #index_ident)?;
                    stream.value(#ident)?;
                    stream.record_tuple_value_end(#tag, #label_ident, #index_ident)?;
                }));

                target = RecordTupleTarget::RecordTuple;
                labeled_field_count += 1;
                indexed_field_count += 1;
            }
            (None, Some(index)) => {
                stream_field.push(quote!({
                    let #index_ident = #index;

                    stream.tuple_value_begin(#tag, #index_ident)?;
                    stream.value(#ident)?;
                    stream.tuple_value_end(#tag, #index_ident)?;
                }));

                target = RecordTupleTarget::Tuple;
                indexed_field_count += 1;
            }
            (Some(label), None) => {
                stream_field.push(quote!({
                    let #label_ident = #label;

                    stream.record_value_begin(#tag, #label_ident)?;
                    stream.value(#ident)?;
                    stream.record_value_end(#tag, #label_ident)?;
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

                let mut #index_ident = 0;

                #(
                    #stream_field
                )*

                stream.record_tuple_end(#tag, #label, #index)?;
            })
        }
        RecordTupleTarget::Tuple => {
            quote!(#path { #(#field_binding,)* } => {
                stream.tuple_begin(#tag, #label, #index, Some(#field_count))?;

                let mut #index_ident = 0;

                #(
                    #stream_field
                )*

                stream.tuple_end(#tag, #label, #index)?;
            })
        }
        RecordTupleTarget::Record => {
            quote!(#path { #(#field_binding,)* } => {
                stream.record_begin(#tag, #label, #index, Some(#field_count))?;

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
    label: Option<&str>,
    index: Option<Index>,
) -> proc_macro2::TokenStream {
    let tag = quote_optional_tag(tag);
    let label = quote_optional_label(label);
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
    label: Option<&str>,
    index: Option<Index>,
) -> proc_macro2::TokenStream {
    let tag = quote_optional_tag(tag);
    let label = quote_optional_label(label);
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
    next_const_index: isize,
    explicit: bool,
}

impl IndexAllocator {
    fn new() -> Self {
        IndexAllocator {
            next_const_index: 0,
            explicit: false,
        }
    }

    fn const_index_of(explicit: isize) -> Index {
        Index::Explicit(quote!(#explicit))
    }

    fn next_const_index(&mut self, explicit: Option<isize>) -> Index {
        if let Some(index) = explicit {
            self.explicit = true;
            self.next_const_index = index + 1;

            Index::Explicit(quote!(#index))
        } else {
            let index = self.next_const_index;
            self.next_const_index += 1;

            if self.explicit {
                Index::Explicit(quote!(#index))
            } else {
                Index::Implicit(quote!(#index))
            }
        }
    }

    fn next_computed_index(&mut self, ident: &syn::Ident, explicit: Option<isize>) -> Index {
        match self.next_const_index(explicit) {
            Index::Implicit(_) => Index::Implicit(quote!({
                let index = #ident;
                #ident += 1;
                index
            })),
            Index::Explicit(index) => Index::Explicit(index),
        }
    }
}

#[derive(Debug, Clone)]
enum Index {
    Implicit(proc_macro2::TokenStream),
    Explicit(proc_macro2::TokenStream),
}
