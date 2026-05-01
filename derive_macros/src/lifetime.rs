use syn::parse::{Parse, ParseStream};
use syn::{Lifetime, WhereClause};

/**
A lifetime specification with optional bounds.
*/
#[derive(Clone)]
pub(crate) struct RefLifetime {
    pub(crate) lifetime: Lifetime,
    pub(crate) bounds: Option<WhereClause>,
}

impl Parse for RefLifetime {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lifetime: syn::Lifetime = input.parse()?;

        let bounds = if input.peek(Token![where]) {
            let bounds: syn::WhereClause = input.parse()?;
            Some(bounds)
        } else {
            None
        };

        Ok(RefLifetime { lifetime, bounds })
    }
}
