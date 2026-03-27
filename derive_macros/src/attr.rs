use std::collections::HashSet;

use syn::{spanned::Spanned, Attribute, Expr, ExprUnary, Lit, Path, UnOp};

use crate::{index::IndexValue, label::LabelValue};

/**
The `tag` attribute.

This attribute specifies a path to an `sval::Tag` to use
for the annotated item.
*/
pub(crate) struct TagAttr;

impl SvalAttribute for TagAttr {
    type Result = syn::Path;

    fn from_expr(&self, expr: &Expr) -> syn::Result<Self::Result> {
        match expr {
            Expr::Lit(lit) => Ok(self.from_lit(&lit.lit)?),
            Expr::Path(path) => Ok(path.path.clone()),
            _ => Err(syn::Error::new(
                expr.span(),
                "invalid `tag`: expected literal or path",
            )),
        }
    }

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Str(s) => s.parse().map_err(|e| {
                let mut r = syn::Error::new(s.span(), "invalid `tag`: expected valid path");
                r.combine(e);

                r
            }),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `tag`: expected string literal",
            )),
        }
    }
}

impl RawAttribute for TagAttr {
    fn key(&self) -> &str {
        "tag"
    }
}

/**
The `data_tag` attribute.

This attribute specifies a path to an `sval::Tag` to use
for the data of the annotated item.
 */
pub(crate) struct DataTagAttr;

impl SvalAttribute for DataTagAttr {
    type Result = syn::Path;

    fn from_expr(&self, expr: &Expr) -> syn::Result<Self::Result> {
        match expr {
            Expr::Lit(lit) => Ok(self.from_lit(&lit.lit)?),
            Expr::Path(path) => Ok(path.path.clone()),
            _ => Err(syn::Error::new(
                expr.span(),
                "invalid `data_tag`: expected literal or path",
            )),
        }
    }

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Str(s) => s.parse().map_err(|e| {
                let mut r = syn::Error::new(s.span(), "invalid `data_tag`: expected valid path");
                r.combine(e);

                r
            }),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `data_tag`: expected string literal",
            )),
        }
    }
}

impl RawAttribute for DataTagAttr {
    fn key(&self) -> &str {
        "data_tag"
    }
}

/**
The `label` attribute.

This attribute specifies an `sval::Label` as a constant
to use for the annotated item.
*/
pub(crate) struct LabelAttr;

impl SvalAttribute for LabelAttr {
    type Result = LabelValue;

    fn from_expr(&self, expr: &Expr) -> syn::Result<Self::Result> {
        match expr {
            Expr::Lit(lit) => Ok(self.from_lit(&lit.lit)?),
            Expr::Path(path) => Ok(LabelValue::Ident(quote!(#path))),
            _ => Err(syn::Error::new(
                expr.span(),
                "invalid `label`: expected literal or path",
            )),
        }
    }

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Str(s) => Ok(LabelValue::Const(s.value())),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `label`: expected string literal",
            )),
        }
    }
}

impl RawAttribute for LabelAttr {
    fn key(&self) -> &str {
        "label"
    }
}

/**
The `index` attribute.

This attribute specifies an `sval::Index` as a constant
to use for the annotated item.
*/
pub(crate) struct IndexAttr;

impl IndexAttr {
    fn const_from_lit(&self, lit: &Lit) -> syn::Result<isize> {
        match lit {
            Lit::Int(n) => n.base10_parse().map_err(|e| {
                let mut r = syn::Error::new(n.span(), "invalid `index`: expected integer");
                r.combine(e);

                r
            }),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `index`: expected integer",
            )),
        }
    }
}

impl SvalAttribute for IndexAttr {
    type Result = IndexValue;

    fn from_expr(&self, expr: &Expr) -> syn::Result<Self::Result> {
        match expr {
            // Take `-` into account
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_),
                expr: inner_expr,
                ..
            }) => {
                if let Expr::Lit(ref lit) = **inner_expr {
                    Ok(IndexValue::Const(-(self.const_from_lit(&lit.lit)?)))
                } else {
                    Err(syn::Error::new(
                        inner_expr.span(),
                        "invalid `index`: expected integer",
                    ))
                }
            }
            Expr::Lit(lit) => Ok(IndexValue::Const(self.const_from_lit(&lit.lit)?)),
            Expr::Path(path) => Ok(IndexValue::Ident(quote!(#path))),
            _ => Err(syn::Error::new(
                expr.span(),
                "invalid `index`: expected literal, path, or integer",
            )),
        }
    }

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        Ok(IndexValue::Const(self.const_from_lit(lit)?))
    }
}

impl RawAttribute for IndexAttr {
    fn key(&self) -> &str {
        "index"
    }
}

/**
The `skip` attribute.

This attribute signals that an item should be skipped
from streaming.
*/
pub(crate) struct SkipAttr;

impl SvalAttribute for SkipAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Bool(b) => Ok(b.value),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `skip`: expected boolean",
            )),
        }
    }
}

impl RawAttribute for SkipAttr {
    fn key(&self) -> &str {
        "skip"
    }
}

/**
The `unlabeled_fields` attribute.

This attribute signals that all fields should be unlabeled.
*/
pub(crate) struct UnlabeledFieldsAttr;

impl SvalAttribute for UnlabeledFieldsAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Bool(b) => Ok(b.value),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `unlabeled_fields`: expected boolean",
            )),
        }
    }
}

impl RawAttribute for UnlabeledFieldsAttr {
    fn key(&self) -> &str {
        "unlabeled_fields"
    }
}

/**
The `unindexed_fields` attribute.

This attribute signals that all fields should be unindexed.
*/
pub(crate) struct UnindexedFieldsAttr;

impl SvalAttribute for UnindexedFieldsAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Bool(b) => Ok(b.value),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `unindexed_fields`: expected boolean",
            )),
        }
    }
}

impl RawAttribute for UnindexedFieldsAttr {
    fn key(&self) -> &str {
        "unindexed_fields"
    }
}

/**
The `unlabeled_variants` attribute.

This attribute signals that all variants should be unlabeled.
*/
pub(crate) struct UnlabeledVariantsAttr;

impl SvalAttribute for UnlabeledVariantsAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Bool(b) => Ok(b.value),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `unlabeled_variants`: expected boolean",
            )),
        }
    }
}

impl RawAttribute for UnlabeledVariantsAttr {
    fn key(&self) -> &str {
        "unlabeled_variants"
    }
}

/**
The `unindexed_variants` attribute.

This attribute signals that all variants should be unindexed.
*/
pub(crate) struct UnindexedVariantsAttr;

impl SvalAttribute for UnindexedVariantsAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Bool(b) => Ok(b.value),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `unindexed_variants`: expected boolean",
            )),
        }
    }
}

impl RawAttribute for UnindexedVariantsAttr {
    fn key(&self) -> &str {
        "unindexed_variants"
    }
}

/**
The `dynamic` attribute.

This attribute signals that an enum should be dynamic.
*/
pub(crate) struct DynamicAttr;

impl SvalAttribute for DynamicAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Bool(b) => Ok(b.value),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `dynamic`: expected boolean",
            )),
        }
    }
}

impl RawAttribute for DynamicAttr {
    fn key(&self) -> &str {
        "dynamic"
    }
}

/**
The `transparent` attribute.

This attribute signals that a newtype should stream its inner field
without wrapping it in a tag.
*/
pub(crate) struct TransparentAttr;

impl SvalAttribute for TransparentAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        match lit {
            Lit::Bool(b) => Ok(b.value),
            _ => Err(syn::Error::new(
                lit.span(),
                "invalid `transparent`: expected boolean",
            )),
        }
    }
}

impl RawAttribute for TransparentAttr {
    fn key(&self) -> &str {
        "transparent"
    }
}

/**
The `flatten` attribute.

This attribute will flatten the fields of a value onto its parent.
 */
pub(crate) struct FlattenAttr;

impl SvalAttribute for FlattenAttr {
    type Result = bool;

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result> {
        #[cfg(not(feature = "flatten"))]
        {
            Err(syn::Error::new(
                lit.span(),
                "the `flatten` attribute can only be used when the `flatten` Cargo feature of `sval_derive` is enabled",
            ))
        }
        #[cfg(feature = "flatten")]
        {
            match lit {
                Lit::Bool(b) => Ok(b.value),
                _ => Err(syn::Error::new(
                    lit.span(),
                    "invalid `flatten`: expected boolean",
                )),
            }
        }
    }
}

impl RawAttribute for FlattenAttr {
    fn key(&self) -> &str {
        "flatten"
    }
}

pub(crate) trait RawAttribute {
    fn key(&self) -> &str;
}

pub(crate) trait SvalAttribute: RawAttribute {
    type Result: 'static;

    fn from_expr(&self, expr: &Expr) -> syn::Result<Self::Result> {
        if let Expr::Lit(lit) = expr {
            Ok(self.from_lit(&lit.lit)?)
        } else {
            Err(syn::Error::new(
                expr.span(),
                format_args!("invalid {}: expected literal", self.key()),
            ))
        }
    }

    fn from_lit(&self, lit: &Lit) -> syn::Result<Self::Result>;
}

pub(crate) fn ensure_empty(ctxt: &str, attrs: &[Attribute]) -> syn::Result<()> {
    // Just ensure the attribute list is empty
    for attr in attrs {
        let Some(meta) = sval_attr(ctxt, attr)? else {
            continue;
        };

        for (value_key, _) in meta {
            return Err(syn::Error::new(
                value_key.span(),
                format_args!("unsupported attribute `{}` on {}", quote!(#value_key), ctxt),
            ));
        }
    }

    Ok(())
}

pub(crate) fn ensure_missing<T: SvalAttribute>(
    ctxt: &str,
    request: T,
    attrs: &[Attribute],
) -> syn::Result<()> {
    let key = request.key().to_owned();

    if let Some((unexpected, _)) = find(ctxt, request.key(), attrs)? {
        return Err(syn::Error::new(
            unexpected.span(),
            format_args!("unsupported attribute `{}` on {}", key, ctxt),
        ));
    }

    Ok(())
}

/**
Check the set of attributes, failing if any are duplicated, or aren't known or supported by
the derive context.
*/
pub(crate) fn check(
    ctxt: &str,
    allowed: &[&dyn RawAttribute],
    attrs: &[Attribute],
) -> syn::Result<()> {
    let mut seen = HashSet::new();

    for attr in attrs {
        let Some(meta) = sval_attr(ctxt, attr)? else {
            continue;
        };

        for (value_key, _) in meta {
            let mut is_valid_attr = false;

            for allowed in allowed {
                let attr_key = allowed.key();

                if value_key.is_ident(attr_key) {
                    is_valid_attr = true;

                    if !seen.insert(attr_key) {
                        return Err(syn::Error::new(
                            value_key.span(),
                            format_args!(
                                "duplicate attribute `{}` on {}",
                                quote!(#value_key),
                                ctxt
                            ),
                        ));
                    }
                }
            }

            if !is_valid_attr {
                return Err(syn::Error::new(
                    value_key.span(),
                    format_args!("unsupported attribute `{}` on {}", quote!(#value_key), ctxt),
                ));
            }
        }
    }

    Ok(())
}

/**
Get the value of an attribute, without checking the set itself for validity.

This function will still fail if the requested attribute is invalid, but won't
handle duplicates, which are expected to have been caught by an earlier call to `check`.
*/
pub(crate) fn get<T: SvalAttribute>(
    ctxt: &str,
    request: T,
    attrs: &[Attribute],
) -> syn::Result<Option<T::Result>> {
    let Some((_, value)) = find(ctxt, request.key(), attrs)? else {
        return Ok(None);
    };

    Ok(Some(request.from_expr(&value)?))
}

fn find(ctxt: &str, request_key: &str, attrs: &[Attribute]) -> syn::Result<Option<(Path, Expr)>> {
    for attr in attrs {
        let Some(meta) = sval_attr(ctxt, attr)? else {
            continue;
        };

        for (value_key, value) in meta {
            if value_key.is_ident(request_key) {
                return Ok(Some((value_key, value)));
            }
        }
    }

    Ok(None)
}

fn sval_attr<'a>(
    ctxt: &'a str,
    attr: &'_ Attribute,
) -> syn::Result<Option<impl IntoIterator<Item = (Path, Expr)> + 'a>> {
    if !attr.path().is_ident("sval") {
        return Ok(None);
    }

    let mut results = Vec::new();
    attr.parse_nested_meta(|meta| {
        let expr: Expr = match meta.value() {
            Ok(value) => value.parse()?,
            // If there isn't a value associated with the item
            // then use the boolean `true`
            Err(_) => syn::parse_quote!(true),
        };

        let path = meta.path;

        results.push((path, expr));

        Ok(())
    })
    .map_err(|e| {
        let mut r = syn::Error::new(
            attr.span(),
            format_args!("failed to parse attribute on {ctxt}"),
        );
        r.combine(e);

        r
    })?;

    Ok(Some(results))
}
