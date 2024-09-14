use syn::{ext::IdentExt, Ident};

#[derive(Debug, Clone)]
pub(crate) enum LabelValue {
    Const(String),
    Ident(proc_macro2::TokenStream),
}

#[derive(Debug, Clone)]
pub(crate) enum Label {
    Implicit(proc_macro2::TokenStream),
    Const(proc_macro2::TokenStream),
    Ident(proc_macro2::TokenStream),
}

fn explicit_label(explicit: LabelValue) -> Label {
    match explicit {
        LabelValue::Const(explicit) => Label::Const(quote!(#explicit)),
        LabelValue::Ident(explicit) => Label::Ident(explicit),
    }
}

fn ident_label(ident: &Ident) -> Label {
    Label::Implicit({
        let ident = ident.unraw().to_string();
        quote!(#ident)
    })
}

pub(crate) fn label_or_ident<'a>(explicit: Option<LabelValue>, ident: &Ident) -> Label {
    explicit
        .map(explicit_label)
        .unwrap_or_else(|| ident_label(ident))
}

pub(crate) fn optional_label_or_ident<'a>(
    explicit: Option<LabelValue>,
    ident: Option<&Ident>,
) -> Option<Label> {
    explicit
        .map(explicit_label)
        .or_else(|| ident.map(ident_label))
}

pub(crate) fn quote_label(label: Label) -> proc_macro2::TokenStream {
    match label {
        Label::Implicit(implicit) => {
            quote!(&sval::Label::new(#implicit).with_tag(&sval::tags::VALUE_IDENT))
        }
        Label::Const(explicit) => quote!(&sval::Label::new(#explicit)),
        Label::Ident(explicit) => quote!(&#explicit),
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
