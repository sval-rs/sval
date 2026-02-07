use syn::Path;

pub(crate) fn quote_optional_tag(tag: Option<&Path>) -> proc_macro2::TokenStream {
    match tag {
        Some(tag) => quote!(sval::__private::option::Option::Some(&#tag)),
        None => quote!(sval::__private::option::Option::None),
    }
}

pub(crate) fn quote_optional_tag_owned(tag: Option<&Path>) -> proc_macro2::TokenStream {
    match tag {
        Some(tag) => quote!(sval::__private::option::Option::Some(#tag)),
        None => quote!(sval::__private::option::Option::None),
    }
}
