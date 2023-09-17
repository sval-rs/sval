use syn::Ident;

#[derive(Debug, Clone)]
pub(crate) enum Label {
    Ident(String),
    Text(String),
}

pub(crate) fn label_or_ident<'a>(explicit: Option<&str>, ident: &Ident) -> Label {
    explicit
        .map(|text| Label::Text(text.to_owned()))
        .unwrap_or_else(|| Label::Ident(ident.to_string()))
}

pub(crate) fn optional_label_or_ident<'a>(
    explicit: Option<&str>,
    ident: Option<&Ident>,
) -> Option<Label> {
    explicit
        .map(|explicit| Label::Text(explicit.to_owned()))
        .or_else(|| ident.map(|ident| Label::Ident(ident.to_string())))
}

pub(crate) fn quote_label(label: Label) -> proc_macro2::TokenStream {
    match label {
        Label::Ident(ident) => quote!(&sval::Label::new(#ident).with_tag(&sval::tags::VALUE_IDENT)),
        Label::Text(text) => quote!(&sval::Label::new(#text)),
    }
}

pub(crate) fn quote_optional_label(label: Option<Label>) -> proc_macro2::TokenStream {
    match label {
        Some(label) => {
            let label = quote_label(label);
            quote!(Some(#label))
        }
        None => quote!(None),
    }
}
