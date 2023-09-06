use syn::Path;

use crate::{
    index::{quote_optional_index, Index},
    label::quote_optional_label,
    tag::quote_optional_tag,
};

pub(crate) fn stream_tag(
    path: proc_macro2::TokenStream,
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
) -> proc_macro2::TokenStream {
    let tag = quote_optional_tag(tag);
    let label = quote_optional_label(label);
    let index = quote_optional_index(index);

    quote!(#path => {
        stream.tag(#tag, #label, #index)?;
    })
}
