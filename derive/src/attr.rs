use syn::{Attribute, DeriveInput, Field, Lit, Meta, MetaList, NestedMeta};

pub(crate) fn field_name(field: &Field) -> String {
    let mut rename = None;

    for list in field.attrs.iter().filter_map(sval_attr) {
        for meta in list.nested {
            if let NestedMeta::Meta(Meta::NameValue(value)) = meta {
                if value.path.is_ident("rename") && rename.is_none() {
                    if let Lit::Str(s) = value.lit {
                        rename = Some(s.value());
                        break;
                    }
                }
            }
        }
    }

    rename.unwrap_or_else(|| field.ident.as_ref().unwrap().to_string())
}

pub(crate) fn container_tag(container: &DeriveInput) -> Option<syn::Path> {
    for list in container.attrs.iter().filter_map(sval_attr) {
        for meta in list.nested {
            if let Some(path) = name_value("tag", &meta) {
                return Some(syn::parse_str(&path).unwrap());
            }
        }
    }

    None
}

pub(crate) fn field_tag(field: &Field) -> Option<syn::Path> {
    for list in field.attrs.iter().filter_map(sval_attr) {
        for meta in list.nested {
            if let Some(path) = name_value("field_tag", &meta) {
                return Some(syn::parse_str(&path).unwrap());
            }
        }
    }

    None
}

pub(crate) fn field_data_tag(field: &Field) -> Option<syn::Path> {
    for list in field.attrs.iter().filter_map(sval_attr) {
        for meta in list.nested {
            if let Some(path) = name_value("tag", &meta) {
                return Some(syn::parse_str(&path).unwrap());
            }
        }
    }

    None
}

fn name_value(name: &str, meta: &NestedMeta) -> Option<String> {
    if let NestedMeta::Meta(Meta::NameValue(value)) = meta {
        if value.path.is_ident(name) {
            if let Lit::Str(ref s) = value.lit {
                return Some(s.value());
            }
        }
    }

    None
}

fn sval_attr(attr: &Attribute) -> Option<MetaList> {
    let segments = &attr.path.segments;
    if !(segments.len() == 1 && segments[0].ident == "sval") {
        return None;
    }

    match attr.parse_meta().ok() {
        Some(Meta::List(list)) => Some(list),
        _ => panic!("unsupported attribute"),
    }
}
