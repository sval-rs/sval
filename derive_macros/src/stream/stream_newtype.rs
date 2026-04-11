use syn::{spanned::Spanned, Field, Ident, Path};

use crate::derive::field_codegen;
use crate::{
    attr,
    derive::ImplStrategy,
    index::{quote_optional_index, Index},
    label::{quote_optional_label, Label},
    tag::quote_optional_tag,
};

pub(crate) fn stream_newtype<B>(
    path: proc_macro2::TokenStream,
    field: &Field,
    impl_block: &B,
    tag: Option<&Path>,
    label: Option<Label>,
    index: Option<Index>,
    transparent: bool,
) -> syn::Result<proc_macro2::TokenStream>
where
    B: ImplStrategy + ?Sized,
{
    attr::check(
        "newtype field",
        &[
            &attr::OuterRefAttr,
            &attr::InnerRefAttr,
            &attr::ComputedAttr,
        ],
        &field.attrs,
    )?;

    let ident = Ident::new("field0", field.span());

    let field_value_tokens = impl_block.quote_stream_value(&ident, field_codegen(&field.attrs)?)?;

    Ok(if transparent {
        quote!(#path(ref #ident) => {
            #field_value_tokens?;
        })
    } else {
        let tag = quote_optional_tag(tag);
        let label = quote_optional_label(label);
        let index = quote_optional_index(index);

        quote!(#path(ref #ident) => {
            stream.tagged_begin(#tag, #label, #index)?;
            #field_value_tokens?;
            stream.tagged_end(#tag, #label, #index)?;
        })
    })
}
