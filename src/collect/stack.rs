use crate::{
    std::{
        borrow,
        marker::PhantomData,
    },
    stream::stack::Stack,
};

use super::Error;

/**
Similar to `BorrowMut<T>`.

This trait produces a `DebugRefMut<'a T>` instead of a `&'a mut T`,
which is zero-sized in release mode.
*/
pub(crate) trait DebugBorrowMut<T: ?Sized> {
    fn borrow_mut(&mut self) -> DebugRefMut<T>;
}

impl<T: ?Sized> DebugBorrowMut<T> for T
where
    T: borrow::BorrowMut<T>,
{
    fn borrow_mut(&mut self) -> DebugRefMut<T> {
        DebugRefMut {
            #[cfg(debug_assertions)]
            ref_mut: borrow::BorrowMut::borrow_mut(self),
            _m: Default::default(),
        }
    }
}

pub(crate) struct DebugRefMut<'a, T: ?Sized> {
    #[cfg(debug_assertions)]
    ref_mut: &'a mut T,
    _m: PhantomData<&'a mut T>,
}

impl<'a, T: ?Sized> DebugBorrowMut<T> for DebugRefMut<'a, T> {
    fn borrow_mut(&mut self) -> DebugRefMut<T> {
        DebugRefMut {
            #[cfg(debug_assertions)]
            ref_mut: self.ref_mut,
            _m: Default::default(),
        }
    }
}

/**
An internal stack that provides additional validation in debug
builds for callers of `Stream`. It ensures:

- Only valid calls to the stream are made.
- The stream is not re-used if it returns an error.

These checks aren't exactly bullet-proof, but can assist consumers
in holding streams correctly during development.
*/
#[derive(Default)]
pub(crate) struct DebugStack {
    #[cfg(debug_assertions)]
    inner: DebugStackInner,
    #[cfg(debug_assertions)]
    poisoned: bool,
}

#[derive(Default)]
pub(crate) struct DebugStackInner {
    #[cfg(debug_assertions)]
    stack: Stack,
    _m: PhantomData<Stack>,
}

impl<'a> DebugRefMut<'a, DebugStack> {
    #[inline]
    pub(crate) fn and_then<R>(
        &mut self,
        mut f: impl FnMut(DebugRefMut<DebugStackInner>) -> Result<R, Error>,
    ) -> Result<R, Error> {
        cfg_debug_stack! {
            if #[debug_assertions] {
                if self.ref_mut.poisoned {
                    return Err(Error::msg("attempt to use a poisoned stream"));
                }

                self.ref_mut.poisoned = true;
                let r = f(self.ref_mut.inner.borrow_mut())?;
                self.ref_mut.poisoned = false;

                Ok(r)
            } else {
                f(DebugStackInner::default().borrow_mut())
            }
        }
    }
}

impl<'a> DebugRefMut<'a, DebugStackInner> {
    #[inline]
    pub fn primitive(&mut self) {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.ref_mut.stack.primitive()
                    .unwrap_or_else(panic_with_expecting(&self.ref_mut.stack, "(debug only) invalid primitive"));
            }
        }
    }

    #[inline]
    pub fn map_begin(&mut self) {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.ref_mut.stack.map_begin()
                    .unwrap_or_else(panic_with_expecting(&self.ref_mut.stack, "(debug only) invalid map begin"));
            }
        }
    }

    #[inline]
    pub fn map_key(&mut self) {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.ref_mut.stack.map_key()
                    .unwrap_or_else(panic_with_expecting(&self.ref_mut.stack, "(debug only) invalid map key"));
            }
        }
    }

    #[inline]
    pub fn map_value(&mut self) {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.ref_mut.stack.map_value()
                    .unwrap_or_else(panic_with_expecting(&self.ref_mut.stack, "(debug only) invalid map value"));
            }
        }
    }

    #[inline]
    pub fn map_end(&mut self) {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.ref_mut.stack.map_end()
                    .unwrap_or_else(panic_with_expecting(&self.ref_mut.stack, "(debug only) invalid map end"));
            }
        }
    }

    #[inline]
    pub fn seq_begin(&mut self) {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.ref_mut.stack.seq_begin()
                    .unwrap_or_else(panic_with_expecting(&self.ref_mut.stack, "(debug only) invalid seq begin"));
            }
        }
    }

    #[inline]
    pub fn seq_elem(&mut self) {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.ref_mut.stack.seq_elem()
                    .unwrap_or_else(panic_with_expecting(&self.ref_mut.stack, "(debug only) invalid seq elem"));
            }
        }
    }

    #[inline]
    pub fn seq_end(&mut self) {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.ref_mut.stack.seq_end()
                    .unwrap_or_else(panic_with_expecting(&self.ref_mut.stack, "(debug only) invalid seq end"));
            }
        }
    }
}

fn panic_with_expecting<'a, T>(stack: &'a Stack, msg: &'static str) -> impl FnOnce(Error) -> T + 'a {
    move |err| panic!("{:?}: {} (expecting: {})", err, msg, stack.current().expecting())
}

#[cfg(test)]
mod tests {
    #[cfg(not(debug_assertions))]
    mod release {
        use super::super::*;

        use crate::std::mem;

        #[test]
        fn debug_stack_is_zero_sized() {
            assert_eq!(0, mem::size_of::<DebugStack>());
        }

        #[test]
        fn debug_stack_ref_is_zero_sized() {
            assert_eq!(0, mem::size_of::<DebugRefMut<DebugStack>>());
        }
    }
}
