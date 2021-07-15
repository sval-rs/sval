#![doc(html_root_url = "https://docs.rs/sval/1.0.0-alpha.5")]
#![no_std]

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! sval_if_alloc {
    (
        if #[cfg(feature = "alloc")]
        {
            $($with:tt)*
        }
        else
        {
            $($without:tt)*
        }
    ) => {
        $($with)*
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "alloc"))]
macro_rules! sval_if_alloc {
    (
        if #[cfg(feature = "alloc")]
        {
            $($with:tt)*
        }
        else
        {
            $($without:tt)*
        }
    ) => {
        $($without)*
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "std")]
macro_rules! sval_if_std {
    (
        if #[cfg(feature = "std")]
        {
            $($with:tt)*
        }
        else
        {
            $($without:tt)*
        }
    ) => {
        $($with)*
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! sval_if_std {
    (
        if #[cfg(feature = "std")]
        {
            $($with:tt)*
        }
        else
        {
            $($without:tt)*
        }
    ) => {
        $($without)*
    };
}

#[doc(inline)]
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use sval_derive::*;

#[cfg(feature = "std")]
#[macro_use]
#[allow(unused_imports)]
extern crate std;

#[macro_use]
#[allow(unused_imports)]
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as alloc_lib;
#[macro_use]
#[allow(unused_imports)]
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate core as core_lib;
#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use crate::alloc_lib::{
        boxed,
        collections,
        rc,
        string,
        vec,
    };

    pub use crate::core_lib::*;
}

#[cfg(all(not(feature = "std"), not(feature = "alloc")))]
#[macro_use]
#[allow(unused_imports)]
extern crate core as std;

#[macro_use]
mod error;

#[cfg(any(test, feature = "test"))]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub mod test;

#[cfg(feature = "fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub mod fmt;

#[cfg(feature = "serde1_lib")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;

pub mod stream;
pub mod value;

#[cfg(feature = "std")]
pub mod owned;

#[doc(inline)]
pub use self::error::Error;

use self::{
    stream::Stream,
    value::Value,
};

pub fn stream<'v>(
    mut stream: impl Stream<'v>,
    value: &'v (impl Value + ?Sized),
) -> Result<(), Error> {
    value.stream(value::Stream::new(&mut stream))?;

    Ok(())
}

pub fn stream_owned<'a>(mut stream: impl Stream<'a>, value: impl Value) -> Result<(), Error> {
    value.stream_owned(value::Stream::new(&mut stream))?;

    Ok(())
}
