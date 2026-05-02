/*!
Lifetime parsing for `#[sval(ref)]` attribute values.

`RefLifetime` is parsed from the string literal in `#[sval(ref = "'a")]` or `#[sval(ref = "'b where 'b: 'a")]`.
*/

use syn::parse::{Parse, ParseStream};
use syn::{Lifetime, WhereClause};

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
