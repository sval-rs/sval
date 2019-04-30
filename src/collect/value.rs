use crate::{
    collect::{
        Collect,
        Error,
    },
    stream::stack::DebugStack,
    value,
};

/**
A value that's known upfront.
*/
pub(crate) struct Value<'a> {
    stack: &'a mut DebugStack,
    value: &'a dyn value::Value,
}

impl<'a> Value<'a> {
    #[inline]
    pub(crate) fn new(value: &'a impl value::Value, stack: &'a mut DebugStack) -> Self {
        Value { stack, value }
    }

    /**
    Stream this value.
    */
    #[inline]
    pub(crate) fn stream(self, mut stream: impl Collect) -> Result<(), Error> {
        let mut stream = value::Stream::new(&mut stream, self.stack);

        stream.any(self.value)?;

        Ok(())
    }
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
    }
}
