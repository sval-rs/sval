/*!
Where-clause construction and generic bound manipulation.
*/

use proc_macro2::TokenStream;
use syn::{Generics, WhereClause, WherePredicate};

use crate::lifetime::RefLifetime;

pub(crate) fn where_clause_with_bound(generics: &Generics, bound: TokenStream) -> WhereClause {
    let new_predicates = generics.type_params().map::<WherePredicate, _>(|param| {
        let param = &param.ident;
        parse_quote!(#param : #bound)
    });

    let mut generics = generics.clone();
    generics
        .make_where_clause()
        .predicates
        .extend(new_predicates);
    generics.where_clause.unwrap()
}

pub(crate) fn is_generic_type_param(ty: &syn::Type, generics: &Generics) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
            let ident = &type_path.path.segments[0].ident;
            generics.type_params().any(|param| &param.ident == ident)
        } else {
            false
        }
    } else {
        false
    }
}

pub(crate) fn build_impl_generics_for_ref(
    generics: &Generics,
    ref_lifetime: &RefLifetime,
) -> Generics {
    let mut impl_generics = generics.clone();
    let lifetime = &ref_lifetime.lifetime;

    // Check if the ValueRef lifetime already exists in generics
    let lifetime_exists = generics.lifetimes().any(|lt| lt.lifetime == *lifetime);

    // 'static is a built-in lifetime — don't add it as a generic parameter
    let is_static = lifetime.ident == "static";

    if !lifetime_exists && !is_static {
        // Add the ValueRef lifetime to impl_generics params
        let lifetime_param: syn::LifetimeParam = parse_quote!(#lifetime);
        impl_generics
            .params
            .push(syn::GenericParam::Lifetime(lifetime_param));
    }

    // Add lifetime bounds if present (from RefLifetime::bounds)
    if let Some(ref bounds) = ref_lifetime.bounds {
        // Merge bounds into the where clause
        if let Some(ref mut where_clause) = impl_generics.where_clause {
            where_clause
                .predicates
                .extend(bounds.predicates.iter().cloned());
        } else {
            impl_generics.where_clause = Some(bounds.clone());
        }
    }

    impl_generics
}
