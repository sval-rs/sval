use crate::{std::convert::TryInto, Result, Stream};

/**
A producer of structured data.
*/
pub trait Value {
    /**
    Stream this value through a [`Stream`].
    */
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result;

    /**
    Try convert this value into a boolean.
    */
    #[inline]
    fn to_bool(&self) -> Option<bool> {
        struct Extract(Option<bool>);

        impl<'sval> Stream<'sval> for Extract {
            fn bool(&mut self, value: bool) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        self.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into a 32bit binary floating point number.
    */
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        struct Extract(Option<f32>);

        impl<'sval> Stream<'sval> for Extract {
            fn f32(&mut self, value: f32) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        self.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into a 64bit binary floating point number.
    */
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        struct Extract(Option<f64>);

        impl<'sval> Stream<'sval> for Extract {
            fn f64(&mut self, value: f64) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        self.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into a signed 8bit integer.
    */
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into a signed 16bit integer.
    */
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into a signed 32bit integer.
    */
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into a signed 64bit integer.
    */
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.to_i128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into a signed 128bit integer.
    */
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        struct Extract(Option<i128>);

        impl<'sval> Stream<'sval> for Extract {
            fn i128(&mut self, value: i128) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        self.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into an unsigned 8bit integer.
    */
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into an unsigned 16bit integer.
    */
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into an unsigned 32bit integer.
    */
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into an unsigned 64bit integer.
    */
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.to_u128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into an unsigned 128bit integer.
    */
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        struct Extract(Option<u128>);

        impl<'sval> Stream<'sval> for Extract {
            fn u128(&mut self, value: u128) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        self.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into a text string.
    */
    #[inline]
    fn to_text(&self) -> Option<&str> {
        struct Extract<'sval> {
            extracted: Option<&'sval str>,
            seen_fragment: bool,
        }

        impl<'sval> Stream<'sval> for Extract<'sval> {
            fn text_begin(&mut self, _: Option<usize>) -> Result {
                Ok(())
            }

            fn text_fragment(&mut self, fragment: &'sval str) -> Result {
                // Allow either independent strings, or fragments of a single borrowed string
                if !self.seen_fragment {
                    self.extracted = Some(fragment);
                    self.seen_fragment = true;
                } else {
                    self.extracted = None;
                }

                Ok(())
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                self.extracted = None;
                self.seen_fragment = true;

                crate::error()
            }

            fn text_end(&mut self) -> Result {
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract {
            extracted: None,
            seen_fragment: false,
        };

        self.stream(&mut extract).ok()?;
        extract.extracted
    }

    /**
    Try convert this value into a bitstring.
    */
    #[inline]
    fn to_binary(&self) -> Option<&[u8]> {
        struct Extract<'sval> {
            extracted: Option<&'sval [u8]>,
            seen_fragment: bool,
        }

        impl<'sval> Stream<'sval> for Extract<'sval> {
            fn binary_begin(&mut self, _: Option<usize>) -> Result {
                Ok(())
            }

            fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
                // Allow either independent bytes, or fragments of a single borrowed byte stream
                if !self.seen_fragment {
                    self.extracted = Some(fragment);
                    self.seen_fragment = true;
                } else {
                    self.extracted = None;
                }

                Ok(())
            }

            fn binary_fragment_computed(&mut self, _: &[u8]) -> Result {
                self.extracted = None;
                self.seen_fragment = true;

                crate::error()
            }

            fn binary_end(&mut self) -> Result {
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract {
            extracted: None,
            seen_fragment: false,
        };

        self.stream(&mut extract).ok()?;
        extract.extracted
    }
}

macro_rules! impl_value_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
                let $bind = self;
                ($($forward)*).stream(stream)
            }

            #[inline]
            fn to_bool(&self) -> Option<bool> {
                let $bind = self;
                ($($forward)*).to_bool()
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                let $bind = self;
                ($($forward)*).to_f32()
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                let $bind = self;
                ($($forward)*).to_f64()
            }

            #[inline]
            fn to_i8(&self) -> Option<i8> {
                let $bind = self;
                ($($forward)*).to_i8()
            }

            #[inline]
            fn to_i16(&self) -> Option<i16> {
                let $bind = self;
                ($($forward)*).to_i16()
            }

            #[inline]
            fn to_i32(&self) -> Option<i32> {
                let $bind = self;
                ($($forward)*).to_i32()
            }

            #[inline]
            fn to_i64(&self) -> Option<i64> {
                let $bind = self;
                ($($forward)*).to_i64()
            }

            #[inline]
            fn to_i128(&self) -> Option<i128> {
                let $bind = self;
                ($($forward)*).to_i128()
            }

            #[inline]
            fn to_u8(&self) -> Option<u8> {
                let $bind = self;
                ($($forward)*).to_u8()
            }

            #[inline]
            fn to_u16(&self) -> Option<u16> {
                let $bind = self;
                ($($forward)*).to_u16()
            }

            #[inline]
            fn to_u32(&self) -> Option<u32> {
                let $bind = self;
                ($($forward)*).to_u32()
            }

            #[inline]
            fn to_u64(&self) -> Option<u64> {
                let $bind = self;
                ($($forward)*).to_u64()
            }

            #[inline]
            fn to_u128(&self) -> Option<u128> {
                let $bind = self;
                ($($forward)*).to_u128()
            }

            #[inline]
            fn to_text(&self) -> Option<&str> {
                let $bind = self;
                ($($forward)*).to_text()
            }

            #[inline]
            fn to_binary(&self) -> Option<&[u8]> {
                let $bind = self;
                ($($forward)*).to_binary()
            }
        }
    };
}

macro_rules! impl_value_ref_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result {
                let $bind = self;
                ($($forward)*).stream_ref(stream)
            }
        }
    };
}

/**
A producer of structured data that stores a reference internally.

This trait is a variant of [`Value`] for wrapper types that keep a reference to a value internally.
In `Value`, the `'sval` lifetime comes from the borrow of `&'sval self`. In `ValueRef`, it comes
from the `'sval` lifetime in the trait itself.

This is a niche API. In general values shouldn't implement it, even if they technically could.
*/
pub trait ValueRef<'sval>: Value {
    /**
    Stream this value through a [`Stream`].
    */
    fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result;
}

impl_value_forward!({impl<'a, T: Value + ?Sized> Value for &'a T} => x => { **x });
impl_value_ref_forward!({impl<'sval, 'a, T: ValueRef<'sval> + ?Sized> ValueRef<'sval> for &'a T} => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_value_forward!({impl<T: Value + ?Sized> Value for Box<T>} => x => { **x });
    impl_value_ref_forward!({impl<'sval, T: ValueRef<'sval> + ?Sized> ValueRef<'sval> for Box<T>} => x => { **x });
}
