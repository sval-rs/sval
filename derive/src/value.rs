/*
Copyright (c) 2018

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
*/

use crate::{
    attr,
    bound,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{
    Data,
    DataStruct,
    DeriveInput,
    Fields,
    Ident,
};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    match attr::derive_provider(&input) {
        attr::DeriveProvider::Sval => derive_from_sval(input),
        attr::DeriveProvider::Serde => derive_from_serde(input),
    }
}

pub(crate) fn derive_from_serde(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let dummy = Ident::new(
        &format!("_IMPL_SVAL_VALUE_FOR_{}", ident),
        Span::call_site(),
    );

    let bound = parse_quote!(sval::value::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            extern crate sval;

            impl #impl_generics sval::value::Value for #ident #ty_generics #bounded_where_clause {
                fn stream(&self, mut stream: sval::value::Stream) -> sval::value::Result {
                    sval::derive_from_serde!(
                        if #[cfg(feature = "serde1_lib")] {
                            stream.owned().any(&sval::serde::v1::to_value(self))
                        }
                        else {
                            compile_error!("#[sval(derive_from = \"serde\")] requires the `serde` feature of `sval`")
                        })
                }
            }
        };
    })
}

pub(crate) fn derive_from_sval(input: DeriveInput) -> TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields,
        _ => panic!("currently only structs with named fields are supported"),
    };

    let ident = input.ident;
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let dummy = Ident::new(
        &format!("_IMPL_SVAL_VALUE_FOR_{}", ident),
        Span::call_site(),
    );

    let fieldname = &fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let fieldstr = fields.named.iter().map(attr::name_of_field);
    let num_fields = fieldname.len();

    let bound = parse_quote!(sval::value::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            extern crate sval;

            impl #impl_generics sval::value::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'s, 'v>(&'v self, mut stream: sval::value::Stream<'s, 'v>) -> sval::value::Result {
                    stream.map_begin(Some(#num_fields))?;

                    #(
                        stream.owned().map_key(&sval::stream::Ident::Static(#fieldstr))?;
                        stream.map_value(&self.#fieldname)?;
                    )*

                    stream.map_end()
                }
            }
        };
    })
}
