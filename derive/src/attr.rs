/*
Copyright (c) 2018

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
*/

use syn::{
    Attribute,
    DeriveInput,
    Field,
    Lit,
    Meta,
    MetaList,
    MetaNameValue,
    NestedMeta,
};

pub(crate) enum DeriveProvider {
    Sval,
    Serde,
}

pub(crate) fn derive_provider(input: &DeriveInput) -> DeriveProvider {
    for list in input.attrs.iter().filter_map(sval_attr) {
        for meta in list.nested {
            match meta {
                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    ref path,
                    lit: Lit::Str(ref s),
                    ..
                })) if path.is_ident("derive_from") && s.value() == "serde" => {
                    return DeriveProvider::Serde;
                }
                _ => panic!("unsupported attribute"),
            }
        }
    }

    DeriveProvider::Sval
}

pub(crate) fn name_of_field(field: &Field) -> String {
    let mut rename = None;

    for list in field.attrs.iter().filter_map(sval_attr) {
        for meta in list.nested {
            if let NestedMeta::Meta(Meta::NameValue(value)) = meta {
                if value.path.is_ident("rename") && rename.is_none() {
                    if let Lit::Str(s) = value.lit {
                        rename = Some(s.value());
                        continue;
                    }
                }
            }
            panic!("unsupported attribute");
        }
    }

    rename.unwrap_or_else(|| field.ident.as_ref().unwrap().to_string())
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
