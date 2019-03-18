use crate::{
    collect::{
        Collect,
        Error,
    },
    std::marker::PhantomData,
    stream::stack,
    value,
};

/**
A value that's known upfront.
*/
pub(crate) struct Value<'a> {
    stack: DebugStack<'a>,
    value: &'a dyn value::Value,
}

impl<'a> Value<'a> {
    #[inline]
    pub(crate) fn new(value: &'a impl value::Value, stack: &'a mut stack::DebugStack) -> Self {
        Value {
            stack: DebugStack::new(stack),
            value,
        }
    }

    /**
    Stream this value.

    The value may only be streamed once.
    Subsequent calls to `stream` may fail.
    */
    #[inline]
    pub(crate) fn stream(&self, mut stream: impl Collect) -> Result<(), Error> {
        let mut stack = self.stack.take()?;

        let mut stream = value::Stream::new(&mut stream, stack.get_mut());

        stream.any(&self.value)?;

        Ok(())
    }
}

// Like `stream::DebugStack`, but we store the stack in a cell we can consume
// It makes it possible to consume a stack from an immutable reference from
// something like `serde::Serialize`.
struct DebugStack<'a> {
    #[cfg(debug_assertions)]
    stack: crate::std::cell::Cell<Option<InnerDebugStack<'a>>>,
    _m: PhantomData<InnerDebugStack<'a>>,
}

impl<'a> DebugStack<'a> {
    #[inline]
    fn new(stack: &'a mut stack::DebugStack) -> Self {
        cfg_debug_stack! {
            if #[debug_assertions] {
                DebugStack {
                    stack: crate::std::cell::Cell::new(Some(InnerDebugStack {
                        stack,
                        _m: PhantomData,
                    })),
                    _m: PhantomData,
                }
            }
            else {
                let _ = stack;

                DebugStack {
                    _m: PhantomData,
                }
            }
        }
    }

    #[inline]
    fn take(&self) -> Result<InnerDebugStack<'a>, Error> {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.stack
                    .take()
                    .ok_or_else(|| Error::msg("attempt to re-use value"))
            }
            else {
                Ok(InnerDebugStack {
                    stack: stack::DebugStack::default(),
                    _m: PhantomData,
                })
            }
        }
    }
}

struct InnerDebugStack<'a> {
    #[cfg(debug_assertions)]
    stack: &'a mut stack::DebugStack,
    #[cfg(not(debug_assertions))]
    stack: stack::DebugStack,
    _m: PhantomData<&'a mut stack::DebugStack>,
}

impl<'a> InnerDebugStack<'a> {
    fn get_mut(&mut self) -> &mut stack::DebugStack {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.stack
            }
            else {
                &mut self.stack
            }
        }
    }
}

#[cfg(all(test, not(debug_assertions)))]
mod tests {
    use super::*;

    use crate::std::mem;

    #[test]
    fn debug_stack_is_zero_sized() {
        assert_eq!(0, mem::size_of::<DebugStack>());
    }
}
