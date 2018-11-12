/*!
Traits for a structured value.
*/

use std::fmt;

#[doc(inline)]
pub use crate::{Visit, VisitSeq, VisitMap, Error};

/**
A value that can be visited.
*/
pub trait Value: fmt::Debug {
    fn visit(&self, visit: Visit) -> Result<(), Error>;
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        (**self).visit(visit)
    }
}
