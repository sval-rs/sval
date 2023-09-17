use syn::{Attribute, Fields, Generics, Ident, Path};

use crate::{
    attr::{self},
    bound,
    index::{Index, IndexAllocator},
    label::label_or_ident,
    stream::{stream_record_tuple, RecordTupleTarget},
    tag::quote_optional_tag_owned,
};

pub(crate) struct StructAttrs {
    tag: Option<Path>,
    label: Option<String>,
    index: Option<isize>,
    unlabeled_values: bool,
    unindexed_values: bool,
}

impl StructAttrs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        attr::check(
            "struct",
            &[
                &attr::TagAttr,
                &attr::LabelAttr,
                &attr::IndexAttr,
                &attr::UnlabeledValuesAttr,
                &attr::UnindexedValuesAttr,
            ],
            attrs,
        );

        let tag = attr::get_unchecked("struct", attr::TagAttr, attrs);
        let label = attr::get_unchecked("struct", attr::LabelAttr, attrs);
        let index = attr::get_unchecked("struct", attr::IndexAttr, attrs);

        let unlabeled_values =
            attr::get_unchecked("struct", attr::UnlabeledValuesAttr, attrs).unwrap_or(false);
        let unindexed_values =
            attr::get_unchecked("struct", attr::UnindexedValuesAttr, attrs).unwrap_or(false);

        StructAttrs {
            tag,
            label,
            index,
            unlabeled_values,
            unindexed_values,
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

    pub(crate) fn unlabeled_values(&self) -> bool {
        self.unlabeled_values
    }

    pub(crate) fn unindexed_values(&self) -> bool {
        self.unindexed_values
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
        Some(label_or_ident(attrs.label(), ident)),
        attrs.index(),
        attrs.unlabeled_values(),
        attrs.unindexed_values(),
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
