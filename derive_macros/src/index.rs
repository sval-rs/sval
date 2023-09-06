pub(crate) struct IndexAllocator {
    next_const_index: isize,
    explicit: bool,
}

impl IndexAllocator {
    pub(crate) fn new() -> Self {
        IndexAllocator {
            next_const_index: 0,
            explicit: false,
        }
    }

    pub(crate) fn const_index_of(explicit: isize) -> Index {
        Index::Explicit(quote!(#explicit))
    }

    pub(crate) fn next_const_index(&mut self, explicit: Option<isize>) -> Index {
        if let Some(index) = explicit {
            self.explicit = true;
            self.next_const_index = index + 1;

            Index::Explicit(quote!(#index))
        } else {
            let index = self.next_const_index;
            self.next_const_index += 1;

            if self.explicit {
                Index::Explicit(quote!(#index))
            } else {
                Index::Implicit(quote!(#index))
            }
        }
    }

    pub(crate) fn next_computed_index(
        &mut self,
        ident: &syn::Ident,
        explicit: Option<isize>,
    ) -> Index {
        match self.next_const_index(explicit) {
            Index::Implicit(_) => Index::Implicit(quote!({
                let index = #ident;
                #ident += 1;
                index
            })),
            Index::Explicit(index) => Index::Explicit(index),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Index {
    Implicit(proc_macro2::TokenStream),
    Explicit(proc_macro2::TokenStream),
}

pub(crate) fn quote_index(index: Index) -> proc_macro2::TokenStream {
    match index {
        Index::Explicit(index) => quote!(&sval::Index::from(#index)),
        Index::Implicit(index) => {
            quote!(&sval::Index::from(#index).with_tag(&sval::tags::VALUE_OFFSET))
        }
    }
}

pub(crate) fn quote_optional_index(index: Option<Index>) -> proc_macro2::TokenStream {
    match index {
        Some(index) => {
            let index = quote_index(index);
            quote!(Some(#index))
        }
        None => quote!(None),
    }
}
