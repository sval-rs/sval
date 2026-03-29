use syn::{Attribute, Generics, Ident};

use crate::{
    attr, bound,
    derive::{self, ImplTokens},
};

pub(crate) struct VoidAttrs {
    lifetime: Option<derive::LifetimeValue>,
}

impl VoidAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        attr::check("void enum", &[&attr::LifetimeAttr], attrs)?;

        let lifetime = attr::get("void enum", attr::LifetimeAttr, attrs)?;

        Ok(VoidAttrs { lifetime })
    }

    pub(crate) fn lifetime(&self) -> Option<derive::LifetimeValue> {
        self.lifetime.clone()
    }
}

pub(crate) fn derive_void<'a, T: ImplTokens>(
    ident: &Ident,
    generics: &Generics,
    attrs: &VoidAttrs,
) -> syn::Result<proc_macro2::TokenStream> {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    Ok(T::impl_tokens(
        impl_generics,
        ident,
        ty_generics,
        &bounded_where_clause,
        attrs.lifetime(),
        quote!({ match *self {} }),
        None,
    ))
}
