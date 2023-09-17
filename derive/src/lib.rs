#[doc(inline)]
pub use sval_derive_macros::*;

pub mod extensions {
    #[cfg(feature = "flatten")]
    pub use sval_flatten as flatten;
}
