use syn::{Attribute, Generics, Ident};

use crate::{attr, bound};

pub(crate) struct VoidAttrs {}

impl VoidAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        attr::ensure_empty("void enum", attrs);

        VoidAttrs {}
    }
}

pub(crate) fn derive_void<'a>(
    ident: &Ident,
    generics: &Generics,
    attrs: &VoidAttrs,
) -> proc_macro2::TokenStream {
    let _ = attrs;

    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    match *self {}
                }
            }
        };
    }
}
