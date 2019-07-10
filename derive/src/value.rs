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

/**
Use an expected `serde::Serialize` implementation to derive `sval::Value`.
*/
pub(crate) fn derive_from_serde(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let dummy = Ident::new(
        &format!("_IMPL_SVAL_VALUE_FOR_{}", ident),
        Span::call_site(),
    );

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream(&self, stream: &mut sval::value::Stream) -> Result<(), sval::value::Error> {
                    sval::derive_from_serde!(
                        if #[cfg(feature = "serde_lib")] {
                            stream.any(sval::serde::to_value(self))
                        }
                        else {
                            compile_error!("#[sval(derive_from = \"serde\")] requires the `serde` feature of `sval`")
                        })
                }
            }
        };
    })
}

/**
Construct an implementation of `sval::Value` based on the structure of the input.
*/
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

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream(&self, stream: &mut sval::value::Stream) -> Result<(), sval::value::Error> {
                    stream.map_begin(Some(#num_fields))?;

                    #(
                        stream.map_key(#fieldstr)?;
                        stream.map_value(&self.#fieldname)?;
                    )*

                    stream.map_end()
                }
            }
        };
    })
}
