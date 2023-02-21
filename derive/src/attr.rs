use syn::{spanned::Spanned, Attribute, Lit, LitBool, Meta, NestedMeta, Path};

pub(crate) struct Tag;

impl SvalAttribute for Tag {
    type Result = syn::Path;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Str(ref s) = lit {
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

pub(crate) struct Label;

impl SvalAttribute for Label {
    type Result = String;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Str(ref s) = lit {
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

pub(crate) struct Index;

impl SvalAttribute for Index {
    type Result = usize;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Int(ref n) = lit {
            n.base10_parse().expect("invalid value")
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for Index {
    fn key(&self) -> &str {
        "index"
    }
}

pub(crate) struct Skip;

impl SvalAttribute for Skip {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> Self::Result {
        if let Lit::Bool(ref b) = lit {
            b.value
        } else {
            panic!("unexpected value")
        }
    }
}

impl RawAttribute for Skip {
    fn key(&self) -> &str {
        "skip"
    }
}

pub(crate) trait RawAttribute {
    fn key(&self) -> &str;
}

pub(crate) trait SvalAttribute: RawAttribute {
    type Result: 'static;

    fn from_lit(&self, lit: &Lit) -> Self::Result;
}

pub(crate) fn container<T: SvalAttribute>(request: T, attrs: &[Attribute]) -> Option<T::Result> {
    get("container", &[&Tag, &Label, &Index], request, attrs)
}

pub(crate) fn named_field<T: SvalAttribute>(request: T, attrs: &[Attribute]) -> Option<T::Result> {
    get("named field", &[&Tag, &Label, &Skip], request, attrs)
}

pub(crate) fn unnamed_field<T: SvalAttribute>(
    request: T,
    attrs: &[Attribute],
) -> Option<T::Result> {
    get("unnamed field", &[&Tag, &Index, &Skip], request, attrs)
}

pub(crate) fn ensure_newtype_field_empty(attrs: &[Attribute]) {
    ensure_empty("newtype field", attrs)
}

fn ensure_empty(ctxt: &str, attrs: &[Attribute]) {
    // Just ensure the attribute list is empty
    for (value_key, _) in attrs
        .iter()
        .filter_map(|attr| sval_attr(ctxt, attr))
        .flatten()
    {
        panic!("unsupported attribute `{}` on {}", quote!(#value_key), ctxt);
    }
}

fn get<T: SvalAttribute>(
    ctxt: &str,
    allowed: &[&dyn RawAttribute],
    request: T,
    attrs: &[Attribute],
) -> Option<T::Result> {
    let request_key = request.key();

    let mut result = None;

    for (value_key, value) in attrs
        .iter()
        .filter_map(|attr| sval_attr(ctxt, attr))
        .flatten()
    {
        let mut is_valid_attr = false;

        for attr in allowed {
            let attr_key = attr.key();

            if value_key.is_ident(attr_key) {
                is_valid_attr = true;
            }
        }

        if !is_valid_attr {
            panic!("unsupported attribute `{}` on {}", quote!(#value_key), ctxt);
        }

        if value_key.is_ident(request_key) {
            if result.is_none() {
                result = Some(request.from_lit(&value));

                // We don't short-circuit here to check other attributes are valid
            } else {
                panic!("duplicate attribute `{}` on {}", quote!(#value_key), ctxt);
            }
        }
    }

    result
}

fn sval_attr<'a>(
    ctxt: &'a str,
    attr: &'_ Attribute,
) -> Option<impl IntoIterator<Item = (Path, Lit)> + 'a> {
    if !attr.path.is_ident("sval") {
        return None;
    }

    match attr.parse_meta().ok() {
        Some(Meta::List(list)) => Some(list.nested.into_iter().map(move |meta| match meta {
            NestedMeta::Meta(Meta::NameValue(value)) => (value.path, value.lit),
            NestedMeta::Meta(Meta::Path(path)) => {
                let lit = Lit::Bool(LitBool::new(true, path.span()));

                (path, lit)
            }
            _ => panic!("unexpected `sval` attribute on {}", ctxt),
        })),
        _ => panic!("unsupported attribute `{}` on {}", quote!(#attr), ctxt),
    }
}
