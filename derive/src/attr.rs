use syn::{Attribute, Lit, Meta, MetaList, MetaNameValue, NestedMeta};

pub(crate) struct Label;
pub(crate) struct Tag;

const ATTRIBUTES: &[&dyn RawAttribute] = &[
    {
        impl SvalAttribute for Label {
            type Result = String;

            fn parse(&self, attr: &MetaNameValue) -> Self::Result {
                if let Lit::Str(ref s) = attr.lit {
                    s.value()
                } else {
                    panic!("unexpected value")
                }
            }
        }

        impl RawAttribute for Label {
            fn key(&self) -> &str {
                "label"
            }
        }

        &Label
    },
    {
        impl SvalAttribute for Tag {
            type Result = syn::Path;

            fn parse(&self, attr: &MetaNameValue) -> Self::Result {
                if let Lit::Str(ref s) = attr.lit {
                    s.parse().expect("invalid value")
                } else {
                    panic!("unexpected value")
                }
            }
        }

        impl RawAttribute for Tag {
            fn key(&self) -> &str {
                "tag"
            }
        }

        &Tag
    },
];

pub(crate) trait RawAttribute {
    fn key(&self) -> &str;
}

pub(crate) trait SvalAttribute: RawAttribute {
    type Result: 'static;

    fn parse(&self, attr: &MetaNameValue) -> Self::Result;
}

pub(crate) fn get<T: SvalAttribute>(get: T, attrs: &[Attribute]) -> Option<T::Result> {
    let get_key = get.key();

    let mut result = None;

    for list in attrs.iter().filter_map(sval_attr) {
        for meta in list.nested {
            if let NestedMeta::Meta(Meta::NameValue(value)) = meta {
                let value_key = &value.path;

                let mut is_valid_attr = false;

                for attr in ATTRIBUTES {
                    let attr_key = attr.key();

                    if value_key.is_ident(attr_key) {
                        is_valid_attr = true;
                    }
                }

                if !is_valid_attr {
                    panic!("unrecognized attribute `{}`", quote!(#value_key));
                }

                if value_key.is_ident(get_key) {
                    if result.is_none() {
                        result = Some(get.parse(&value));

                        // We don't short-circuit here to check other attributes are valid
                    } else {
                        panic!("duplicate attribute `{}`", quote!(#value_key));
                    }
                }
            } else {
                panic!("unexpected `sval` attribute")
            }
        }
    }

    result
}

fn sval_attr(attr: &Attribute) -> Option<MetaList> {
    if !attr.path.is_ident("sval") {
        return None;
    }

    match attr.parse_meta().ok() {
        Some(Meta::List(list)) => Some(list),
        _ => panic!("unsupported attribute `{}`", quote!(#attr)),
    }
}
