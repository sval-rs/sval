#![no_std]

#[macro_use]
#[allow(unused_imports)]
#[cfg(feature = "alloc")]
extern crate alloc as alloc_lib;
#[macro_use]
#[allow(unused_imports)]
#[cfg(feature = "alloc")]
extern crate core as core_lib;
#[cfg(feature = "alloc")]
mod std {
    pub use crate::alloc_lib::vec;

    pub use crate::core_lib::*;
}

#[cfg(not(feature = "alloc"))]
#[macro_use]
#[allow(unused_imports)]
extern crate core as std;

mod error;

pub mod stack;
pub mod stack2;

pub use self::error::Error;
