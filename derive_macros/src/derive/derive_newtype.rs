use std::borrow::Cow;

use syn::{Attribute, Field, Generics, Ident, Path};

use crate::{
    attr::{self, SvalAttribute},
    bound,
    index::{Index, IndexAllocator},
    label::label_or_ident,
    stream::stream_newtype,
    tag::quote_optional_tag_owned,
};

/**
Get an attribute that is applicable to a newtype struct.
*/
fn newtype_container<T: SvalAttribute>(request: T, attrs: &[Attribute]) -> Option<T::Result> {
    attr::get(
        "newtype",
        &[&attr::TagAttr, &attr::LabelAttr, &attr::IndexAttr],
        request,
        attrs,
    )
}

pub(crate) struct NewtypeAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
}

impl NewtypeAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        let tag = newtype_container(attr::TagAttr, attrs);
        let label = newtype_container(attr::LabelAttr, attrs);
        let index = newtype_container(attr::IndexAttr, attrs);

        NewtypeAttrs { tag, label, index }
    }

    pub(crate) fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    pub(crate) fn label(&self, ident: &Ident) -> Cow<str> {
        label_or_ident(self.label.as_deref(), ident)
    }

    pub(crate) fn index(&self) -> Option<Index> {
        self.index.map(IndexAllocator::const_index_of)
    }
}

pub(crate) fn derive_newtype<'a>(
    ident: &Ident,
    generics: &Generics,
    field: &Field,
    attrs: &NewtypeAttrs,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_newtype(
        quote!(#ident),
        field,
        attrs.tag(),
        Some(&*attrs.label(ident)),
        attrs.index(),
    );

    let tag = quote_optional_tag_owned(attrs.tag());

    quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    match self {
                        #match_arm
                    }

                    Ok(())
                }

                fn tag(&self) -> Option<sval::Tag> {
                    #tag
                }
            }
        };
    }
}
