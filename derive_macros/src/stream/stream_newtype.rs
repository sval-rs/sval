use syn::{Field, Path};

use crate::{
    attr,
    index::{quote_optional_index, Index},
    label::quote_optional_label,
    tag::quote_optional_tag,
};

pub(crate) fn stream_newtype(
    path: proc_macro2::TokenStream,
    field: &Field,
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
    transparent: bool,
) -> proc_macro2::TokenStream {
    attr::ensure_empty("newtype field", &field.attrs);

    if transparent {
        quote!(#path(ref field0) => {
            stream.value(field0)?;
        })
    } else {
        let tag = quote_optional_tag(tag);
        let label = quote_optional_label(label);
        let index = quote_optional_index(index);

        quote!(#path(ref field0) => {
            stream.tagged_begin(#tag, #label, #index)?;
            stream.value(field0)?;
            stream.tagged_end(#tag, #label, #index)?;
        })
    }
}
