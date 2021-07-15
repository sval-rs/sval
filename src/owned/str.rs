pub struct OwnedStr(StringContainer<OwnedContainer<str>>);

#[derive(Clone)]
pub(crate) struct InlineString<T = OwnedContainer<str>>(InlineStringInner<T>);

// Deliberately chosen so that capacity + 1 (for the initialized len) + 1 (for the discriminant) = size_of::<String>()
const INLINE_CAPACITY: usize = 22;

#[derive(Clone)]
enum InlineStringInner<T> {
    Inline(u8, [u8; INLINE_CAPACITY]),
    Shared(T),
}

impl<'a, T> From<&'a str> for InlineString<T>
where
    T: From<&'a str>,
{
    fn from(s: &'a str) -> InlineString<T> {
        let src = s.as_bytes();

        InlineString(if src.len() <= INLINE_CAPACITY {
            // NOTE: We could use `MaybeUninit` here, but it's not really faster
            // and the complexity doesn't seem worth it.
            let mut dst = [0; INLINE_CAPACITY];

            let src_ptr = src.as_ptr();
            let dst_ptr = (&mut dst[..]).as_mut_ptr();

            // SAFETY: The `src` is a valid, initialized `str`
            // The `dst` has enough capacity for `src.len()` bytes
            unsafe {
                ptr::copy_nonoverlapping(src_ptr, dst_ptr, src.len());
            }

            // Because `src.len()` is less than 255 we can convert it to a `u8`
            InlineStringInner::Inline(src.len() as u8, dst)
        } else {
            InlineStringInner::Shared(s.into())
        })
    }
}

impl<T> From<String> for InlineString<T>
where
    T: From<String>,
{
    fn from(s: String) -> InlineString<T> {
        InlineString(InlineStringInner::Shared(s.into()))
    }
}

impl<T> Deref for InlineString<T>
where
    T: Deref<Target = str>,
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self.0 {
            InlineStringInner::Inline(len, ref buf) => {
                // SAFETY: The written portion of `buf[..len]` is a valid UTF8 string
                // SAFETY: `len` is within the bounds of `buf`
                unsafe { str::from_utf8_unchecked(buf.get_unchecked(0..len as usize)) }
            }
            InlineStringInner::Shared(ref s) => &*s,
        }
    }
}
