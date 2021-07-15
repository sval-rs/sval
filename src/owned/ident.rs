pub struct OwnedIdent(Inner);

enum Inner {
    Ident {
        kind: Option<OwnedIdent>,
        ident: OwnedIdent,
    },
    Id {
        kind: Option<OwnedIdent>,
        id: u64,
    },
    Full {
        kind: Option<OwnedIdent>,
        ident: OwnedIdent,
        id: u64,
    },
}
