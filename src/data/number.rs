use crate::{tags, Stream, Value};

macro_rules! int {
    ($($fi:ident => $i:ty, $fu:ident => $u:ty,)*) => {
        $(
            pub(crate) fn $fi<'sval>(v: $i, stream: &mut (impl Stream<'sval> + ?Sized)) -> crate::Result {
                stream.tagged_begin(Some(&tags::NUMBER), None, None)?;

                crate::stream_display(stream, v).map_err(|_| crate::Error::new())?;

                stream.tagged_end(Some(&tags::NUMBER), None, None)
            }

            pub(crate) fn $fu<'sval>(v: $u, stream: &mut (impl Stream<'sval> + ?Sized)) -> crate::Result {
                stream.tagged_begin(Some(&tags::NUMBER), None, None)?;

                crate::stream_display(stream, v).map_err(|_| crate::Error::new())?;

                stream.tagged_end(Some(&tags::NUMBER), None, None)
            }
        )*
    };
}

macro_rules! convert {
    ($(
        $convert:ident => $ty:ident,
    )+) => {
        $(
            impl Value for $ty {
                fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> crate::Result {
                    stream.$ty(*self)
                }

                fn $convert(&self) -> Option<$ty> {
                    Some(*self)
                }
            }
        )+
    };
}

int!(
    stream_i128 => i128,
    stream_u128 => u128,
);

convert!(
    to_u8 => u8,
    to_u16 => u16,
    to_u32 => u32,
    to_u64 => u64,
    to_u128 => u128,
    to_i8 => i8,
    to_i16 => i16,
    to_i32 => i32,
    to_i64 => i64,
    to_i128 => i128,
    to_f32 => f32,
    to_f64 => f64,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_cast() {
        assert_eq!(Some(1u8), 1u8.to_u8());
        assert_eq!(Some(2u16), 2u16.to_u16());
        assert_eq!(Some(3u32), 3u32.to_u32());
        assert_eq!(Some(4u64), 4u64.to_u64());
        assert_eq!(Some(42u128), 42u128.to_u128());

        assert_eq!(Some(1i8), 1i8.to_i8());
        assert_eq!(Some(2i16), 2i16.to_i16());
        assert_eq!(Some(3i32), 3i32.to_i32());
        assert_eq!(Some(4i64), 4i64.to_i64());
        assert_eq!(Some(42i128), 42i128.to_i128());

        assert_eq!(Some(3f32), 3f32.to_f32());
        assert_eq!(Some(4f64), 4f64.to_f64());
    }
}
