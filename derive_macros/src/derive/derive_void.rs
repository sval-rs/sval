use syn::{Attribute, Generics, Ident};

use crate::{
    attr,
    derive::{ImplStrategy, ImplValue},
};

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

    ImplValue::new(None).quote_impl(ident, generics, quote!({ match *self {} }))
}
