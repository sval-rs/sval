use std::borrow::Cow;

use syn::{Attribute, Generics, Ident, Path};

use crate::{
    attr::{self, SvalAttribute},
    bound,
    index::{Index, IndexAllocator},
    label::label_or_ident,
    stream::stream_tag,
    tag::quote_optional_tag_owned,
};

/**
Get an attribute that is applicable to a unit struct.
*/
fn unit_container<T: SvalAttribute>(request: T, attrs: &[Attribute]) -> Option<T::Result> {
    attr::get(
        "unit struct",
        &[&attr::TagAttr, &attr::LabelAttr, &attr::IndexAttr],
        request,
        attrs,
    )
}

pub(crate) struct UnitStructAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
}

impl UnitStructAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        let tag = unit_container(attr::TagAttr, attrs);
        let label = unit_container(attr::LabelAttr, attrs);
        let index = unit_container(attr::IndexAttr, attrs);

        UnitStructAttrs { tag, label, index }
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

pub(crate) fn derive_unit_struct<'a>(
    ident: &Ident,
    generics: &Generics,
    attrs: &UnitStructAttrs,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_tag(
        quote!(_),
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
