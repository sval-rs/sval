use std::borrow::Cow;

use syn::{Attribute, Fields, Generics, Ident, Path};

use crate::{
    attr::{self, SvalAttribute},
    bound,
    index::{Index, IndexAllocator},
    label::label_or_ident,
    stream::{stream_record_tuple, RecordTupleTarget},
    tag::quote_optional_tag_owned,
};

/**
Get an attribute that is applicable to a struct.
*/
pub(crate) fn struct_container<T: SvalAttribute>(
    request: T,
    attrs: &[Attribute],
) -> Option<T::Result> {
    attr::get(
        "struct",
        &[
            &attr::TagAttr,
            &attr::LabelAttr,
            &attr::IndexAttr,
            &attr::UnlabeledFieldsAttr,
            &attr::UnindexedFieldsAttr,
        ],
        request,
        attrs,
    )
}

pub(crate) struct StructAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
    unlabeled_fields: bool,
    unindexed_fields: bool,
}

impl StructAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        let tag = struct_container(attr::TagAttr, attrs);
        let label = struct_container(attr::LabelAttr, attrs);
        let index = struct_container(attr::IndexAttr, attrs);

        let unlabeled_fields = struct_container(attr::UnlabeledFieldsAttr, attrs).unwrap_or(false);
        let unindexed_fields = struct_container(attr::UnindexedFieldsAttr, attrs).unwrap_or(false);

        StructAttrs {
            tag,
            label,
            index,
            unlabeled_fields,
            unindexed_fields,
        }
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

    pub(crate) fn unlabeled_fields(&self) -> bool {
        self.unlabeled_fields
    }

    pub(crate) fn unindexed_fields(&self) -> bool {
        self.unindexed_fields
    }
}

pub(crate) fn derive_struct<'a>(
    ident: &Ident,
    generics: &Generics,
    fields: &Fields,
    attrs: &StructAttrs,
) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let (fields, target) = match fields {
        Fields::Named(ref fields) => (&fields.named, RecordTupleTarget::named_fields()),
        Fields::Unnamed(ref fields) => (&fields.unnamed, RecordTupleTarget::unnamed_fields()),
        _ => unreachable!(),
    };

    let match_arm = stream_record_tuple(
        quote!(#ident),
        fields.iter(),
        target,
        attrs.tag(),
        Some(&*attrs.label(ident)),
        attrs.index(),
        attrs.unlabeled_fields(),
        attrs.unindexed_fields(),
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
