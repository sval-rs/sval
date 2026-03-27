use syn::{Attribute, Generics, Ident};

use crate::{attr, bound, derive::impl_tokens};

pub(crate) struct VoidAttrs {}

impl VoidAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        attr::ensure_empty("void enum", attrs)?;

        Ok(VoidAttrs {})
    }
}

pub(crate) fn derive_void<'a>(
    ident: &Ident,
    generics: &Generics,
    attrs: &VoidAttrs,
) -> syn::Result<proc_macro2::TokenStream> {
    let _ = attrs;

    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    Ok(impl_tokens(
        impl_generics,
        ident,
        ty_generics,
        &bounded_where_clause,
        quote!({ match *self {} }),
        None,
    ))
}
