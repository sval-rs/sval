use syn::{Attribute, Field, Generics, Ident, Path};

use crate::{
    attr::{self},
    bound,
    index::{Index, IndexAllocator},
    label::label_or_ident,
    stream::stream_newtype,
    tag::quote_optional_tag_owned,
};

pub(crate) struct NewtypeAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
    transparent: bool,
}

impl NewtypeAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        attr::check(
            "newtype",
            &[
                &attr::TagAttr,
                &attr::LabelAttr,
                &attr::IndexAttr,
                &attr::TransparentAttr,
            ],
            attrs,
        );

        let tag = attr::get_unchecked("newtype", attr::TagAttr, attrs);
        let label = attr::get_unchecked("newtype", attr::LabelAttr, attrs);
        let index = attr::get_unchecked("newtype", attr::IndexAttr, attrs);
        let transparent =
            attr::get_unchecked("newtype", attr::TransparentAttr, attrs).unwrap_or(false);

        if transparent {
            assert!(tag.is_none(), "transparent values cannot have tags");
            assert!(label.is_none(), "transparent values cannot have labels");
            assert!(index.is_none(), "transparent values cannot have indexes");
        }

        NewtypeAttrs {
            tag,
            label,
            index,
            transparent,
        }
    }

    pub(crate) fn tag(&self) -> Option<&Path> {
        self.tag.as_ref()
    }

    pub(crate) fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub(crate) fn index(&self) -> Option<Index> {
        self.index.map(IndexAllocator::const_index_of)
    }

    pub(crate) fn transparent(&self) -> bool {
        self.transparent
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
        Some(&*label_or_ident(attrs.label(), ident)),
        attrs.index(),
        attrs.transparent(),
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
