use crate::{attr, bound};
use proc_macro::TokenStream;
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Generics, Ident, Path, Variant,
};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    let tag = attr::container_tag(&input);

    match &input.data {
        Data::Struct(DataStruct { ref fields, .. }) if fields.len() == 0 => {
            derive_unit_struct(tag.as_ref(), &input.ident, &input.generics)
        }
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => derive_struct(tag.as_ref(), &input.ident, &input.generics, fields),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) if fields.unnamed.len() == 1 => {
            derive_newtype(tag.as_ref(), &input.ident, &input.generics)
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) => derive_tuple(tag.as_ref(), &input.ident, &input.generics, fields),
        Data::Enum(DataEnum { variants, .. }) => {
            derive_enum(tag.as_ref(), &input.ident, &input.generics, variants.iter())
        }
        _ => panic!("unimplemented"),
    }
}

fn derive_struct<'a>(
    tag: Option<&Path>,
    ident: &Ident,
    generics: &Generics,
    fields: &FieldsNamed,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_record(quote!(#ident), tag, &ident, None, fields);

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
            }
        };
    })
}

fn derive_unit_struct<'a>(tag: Option<&Path>, ident: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_tag(quote!(#ident), tag, &ident, None);

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
            }
        };
    })
}

fn derive_newtype<'a>(tag: Option<&Path>, ident: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_newtype(quote!(#ident), tag, &ident, None);

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
            }
        };
    })
}

fn derive_tuple<'a>(
    tag: Option<&Path>,
    ident: &Ident,
    generics: &Generics,
    fields: &FieldsUnnamed,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_tuple(quote!(#ident), tag, &ident, None, fields);

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
            }
        };
    })
}

fn derive_enum<'a>(
    tag: Option<&Path>,
    ident: &Ident,
    generics: &Generics,
    variants: impl Iterator<Item = &'a Variant> + 'a,
) -> TokenStream {
    let (enum_tag, enum_label, enum_index) = quote_tag_label_index(tag, ident, None);

    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let mut variant_match_arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        variant_match_arms.push(match variant.fields {
            Fields::Named(ref fields) => stream_record(
                quote!(#ident :: #variant_ident),
                tag,
                &variant.ident,
                Some(variant_match_arms.len()),
                fields,
            ),
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => stream_newtype(
                quote!(#ident :: #variant_ident),
                tag,
                &variant.ident,
                Some(variant_match_arms.len()),
            ),
            Fields::Unnamed(ref fields) => stream_tuple(
                quote!(#ident :: #variant_ident),
                tag,
                &variant.ident,
                Some(variant_match_arms.len()),
                fields,
            ),
            Fields::Unit => stream_tag(
                quote!(#ident :: #variant_ident),
                tag,
                &variant.ident,
                Some(variant_match_arms.len()),
            ),
        });
    }

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
            }
        };
    })
}

fn stream_record(
    path: proc_macro2::TokenStream,
    tag: Option<&Path>,
    label: &Ident,
    index: Option<usize>,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let (tag, label, index) = quote_tag_label_index(tag, label, index);

    let mut field_count = 0usize;
    let mut field_ident = Vec::new();
    let mut stream_field = Vec::new();

    for field in &fields.named {
        let label = attr::field_name(field);
        let label = quote!(&sval::Label::new(#label));

        let ident = &field.ident;

        let field_tag = quote_tag(attr::field_tag(field).as_ref());

        stream_field.push(if let Some(tag) = attr::field_data_tag(field) {
            quote!({
                stream.record_value_begin(#field_tag, #label)?;
                stream.tagged_begin(Some(&#tag), None, None)?;
                stream.value(#ident)?;
                stream.tagged_end(Some(&#tag), None, None)?;
                stream.record_value_end(#field_tag, #label)?;
            })
        } else {
            quote!({
                stream.record_value_begin(#field_tag, #label)?;
                stream.value(#ident)?;
                stream.record_value_end(#field_tag, #label)?;
            })
        });

        field_ident.push(ident.clone());
        field_count += 1;
    }

    quote!(#path { #(ref #field_ident,)* } => {
        stream.record_begin(#tag, #label, #index, Some(#field_count))?;

        #(
            #stream_field
        )*

        stream.record_end(#tag, #label, #index)?;
    })
}

fn stream_newtype(
    path: proc_macro2::TokenStream,
    tag: Option<&Path>,
    label: &Ident,
    index: Option<usize>,
) -> proc_macro2::TokenStream {
    let (tag, label, index) = quote_tag_label_index(tag, label, index);

    quote!(#path(ref field0) => {
        stream.tagged_begin(#tag, #label, #index)?;
        stream.value(field0)?;
        stream.tagged_end(#tag, #label, #index)?;
    })
}

fn stream_tuple(
    path: proc_macro2::TokenStream,
    tag: Option<&Path>,
    label: &Ident,
    index: Option<usize>,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let (tag, label, index) = quote_tag_label_index(tag, label, index);

    let mut field_ident = Vec::new();
    let mut stream_field = Vec::new();
    let mut field_count = 0usize;

    for field in &fields.unnamed {
        let index = field_count;

        let ident = Ident::new(&format!("field{}", field_count), field.span());

        let field_tag = quote_tag(attr::field_tag(field).as_ref());

        stream_field.push(if let Some(tag) = attr::field_data_tag(field) {
            quote!({
                stream.tuple_value_begin(#field_tag, &sval::Index::new(#index))?;
                stream.tagged_begin(Some(&#tag), None, None)?;
                stream.value(#ident)?;
                stream.tagged_end(Some(&#tag), None, None)?;
                stream.tuple_value_end(#field_tag, &sval::Index::new(#index))?;
            })
        } else {
            quote!({
                stream.tuple_value_begin(#field_tag, &sval::Index::new(#index))?;
                stream.value(#ident)?;
                stream.tuple_value_end(#field_tag, &sval::Index::new(#index))?;
            })
        });

        field_ident.push(ident);
        field_count += 1;
    }

    quote!(#path(#(ref #field_ident,)*) => {
        stream.tuple_begin(#tag, #label, #index, Some(#field_count))?;

        #(
            #stream_field
        )*

        stream.tuple_end(#tag, #label, #index)?;
    })
}

fn stream_tag(
    path: proc_macro2::TokenStream,
    tag: Option<&Path>,
    label: &Ident,
    index: Option<usize>,
) -> proc_macro2::TokenStream {
    let (tag, label, index) = quote_tag_label_index(tag, label, index);

    quote!(#path => {
        stream.tag(#tag, #label, #index)?;
    })
}

fn quote_tag_label_index(
    tag: Option<&Path>,
    label: &Ident,
    index: Option<usize>,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let label = label.to_string();

    let tag = quote_tag(tag);

    let label = quote!(Some(&sval::Label::new(#label)));

    let index = match index {
        Some(index) => quote!(Some(&sval::Index::new(#index))),
        None => quote!(None),
    };

    (tag, label, index)
}

fn quote_tag(tag: Option<&Path>) -> proc_macro2::TokenStream {
    match tag {
        Some(tag) => quote!(Some(&#tag)),
        None => quote!(None),
    }
}
