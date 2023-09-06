use syn::{Attribute, Field, Path};

use crate::{
    attr,
    index::{quote_optional_index, Index},
    label::quote_optional_label,
    tag::quote_optional_tag,
};

/**
Ensure that no attributes are applied to a newtype field.
*/
fn ensure_newtype_field_empty(attrs: &[Attribute]) {
    attr::ensure_empty("newtype field", attrs)
}

pub(crate) fn stream_newtype(
    path: proc_macro2::TokenStream,
    field: &Field,
    tag: Option<&Path>,
    label: Option<&str>,
    index: Option<Index>,
) -> proc_macro2::TokenStream {
    ensure_newtype_field_empty(&field.attrs);

    let tag = quote_optional_tag(tag);
    let label = quote_optional_label(label);
    let index = quote_optional_index(index);

    quote!(#path(ref field0) => {
        stream.tagged_begin(#tag, #label, #index)?;
        stream.value(field0)?;
        stream.tagged_end(#tag, #label, #index)?;
    })
}
