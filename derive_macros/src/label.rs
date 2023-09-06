use std::borrow::Cow;

use syn::Ident;

pub(crate) fn label_or_ident<'a>(label: Option<&'a str>, ident: &'_ Ident) -> Cow<'a, str> {
    label
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(ident.to_string()))
}

pub(crate) fn quote_label(label: &str) -> proc_macro2::TokenStream {
    quote!(&sval::Label::new(#label))
}

pub(crate) fn quote_optional_label(label: Option<&str>) -> proc_macro2::TokenStream {
    match label {
        Some(label) => {
            let label = quote_label(label);
            quote!(Some(#label))
        }
        None => quote!(None),
    }
}
