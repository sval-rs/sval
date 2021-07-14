/*!
Owned values.
*/

use crate::{
    std::{
        boxed::Box,
        fmt,
        ops::Deref,
        ptr,
        str,
        string::{
            String,
            ToString,
        },
        vec::Vec,
    },
    stream::{
        self,
        Stream,
    },
    value::{
        self,
        Value,
    },
};

#[cfg(feature = "std")]
use crate::std::sync::Arc;

/**
An owned, immutable value.

Owned values are safe to share and are cheap to clone.

Add the `std` feature to your `Cargo.toml` to enable this type:

```toml,no_run
[dependencies.sval]
features = ["std"]
```
*/
#[derive(Clone)]
pub struct OwnedValue(ValueInner);

impl OwnedValue {
    /**
    Get an owned value from an arbitrary [`Value`].

    The given value doesn't need to be `Send + Sync + 'static`.

    Some primitive types can be converted into an `OwnedValue`
    for free. These types have a corresponding `From` implementation.

    The structure of the given value will be streamed into
    a sharable representation. That means this method is more
    expensive for more complex values.

    Prefer the `From` impls and `from_shared` method where possible.

    [`Value`]: struct.Value.html
    */
    pub fn collect(v: impl Value) -> Self {
        // Try get a primitive first
        // If the value is a simple primitive that can
        // be represented in a single token then we can avoid
        // allocating for it.
        if let Some(primitive) = PrimitiveBuf::collect(&v) {
            return OwnedValue(ValueInner::Primitive(primitive));
        }

        TokenBuf::collect(v)
            .map(|tokens| OwnedValue(ValueInner::Stream(tokens.into())))
            .unwrap_or_else(|err| OwnedValue(ValueInner::Error(err.to_string().into())))
    }

    /**
    Get an owned value from an already shared [`Value`].

    The given value must be `Send + Sync + 'static`.

    [`Value`]: struct.Value.html
    */
    #[cfg(feature = "std")]
    pub fn from_shared(v: impl Into<Arc<dyn Value + Send + Sync>>) -> Self {
        OwnedValue(ValueInner::Shared(v.into()))
    }
}

#[derive(Clone)]
enum ValueInner {
    Error(Box<str>),
    #[cfg(feature = "std")]
    Shared(SharedContainer<dyn Value + Send + Sync>),
    Primitive(Primitive),
    Stream(SharedContainer<[Token]>),
}

impl Value for OwnedValue {
    fn stream<'s, 'v>(&'v self, mut stream: value::Stream<'s, 'v>) -> value::Result {
        match self.0 {
            ValueInner::Error(ref e) => Err(crate::Error::custom(e)),
            #[cfg(feature = "std")]
            ValueInner::Shared(ref v) => v.stream_owned(stream.borrowed()),
            ValueInner::Primitive(ref v) => v.stream_owned(stream.borrowed()),
            ValueInner::Stream(ref v) => {
                for token in v.iter() {
                    token.stream_owned(stream.borrowed())?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Debug for OwnedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "fmt")]
        {
            crate::fmt::debug(f, self)
        }

        #[cfg(not(feature = "fmt"))]
        {
            f.debug_struct("OwnedValue").finish()
        }
    }
}

impl From<usize> for OwnedValue {
    fn from(v: usize) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v as u64)))
    }
}

impl From<u8> for OwnedValue {
    fn from(v: u8) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v as u64)))
    }
}

impl From<u16> for OwnedValue {
    fn from(v: u16) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v as u64)))
    }
}

impl From<u32> for OwnedValue {
    fn from(v: u32) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v as u64)))
    }
}

impl From<u64> for OwnedValue {
    fn from(v: u64) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v)))
    }
}

impl From<u128> for OwnedValue {
    fn from(v: u128) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::BigUnsigned(v)))
    }
}

impl From<isize> for OwnedValue {
    fn from(v: isize) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v as i64)))
    }
}

impl From<i8> for OwnedValue {
    fn from(v: i8) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v as i64)))
    }
}

impl From<i16> for OwnedValue {
    fn from(v: i16) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v as i64)))
    }
}

impl From<i32> for OwnedValue {
    fn from(v: i32) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v as i64)))
    }
}

impl From<i64> for OwnedValue {
    fn from(v: i64) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v)))
    }
}

impl From<i128> for OwnedValue {
    fn from(v: i128) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::BigSigned(v)))
    }
}

impl From<f32> for OwnedValue {
    fn from(v: f32) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Float(v as f64)))
    }
}

impl From<f64> for OwnedValue {
    fn from(v: f64) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Float(v)))
    }
}

impl From<bool> for OwnedValue {
    fn from(v: bool) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Bool(v)))
    }
}

impl From<char> for OwnedValue {
    fn from(v: char) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Char(v)))
    }
}

impl<'a> From<&'a str> for OwnedValue {
    fn from(v: &'a str) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Str(v.into())))
    }
}

impl From<String> for OwnedValue {
    fn from(v: String) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Str(v.into())))
    }
}

type OwnedContainer<T> = Box<T>;

#[cfg(feature = "std")]
type SharedContainer<T> = Arc<T>;
#[cfg(not(feature = "std"))]
type SharedContainer<T> = OwnedContainer<T>;

type StringContainer<T> = InlineString<T>;

#[derive(Clone)]
pub(crate) struct InlineString<T = OwnedContainer<str>>(InlineStringInner<T>);
// Deliberately chosen so that capacity + 1 (for the initialized len) + 1 (for the discriminant) = size_of::<String>()
const SHARED_STR_INLINE_CAPACITY: usize = 22;

#[derive(Clone)]
enum InlineStringInner<T> {
    Inline(u8, [u8; SHARED_STR_INLINE_CAPACITY]),
    Shared(T),
}

impl<'a, T> From<&'a str> for InlineString<T>
where
    T: From<&'a str>,
{
    fn from(s: &'a str) -> InlineString<T> {
        let src = s.as_bytes();

        InlineString(if src.len() <= SHARED_STR_INLINE_CAPACITY {
            // NOTE: We could use `MaybeUninit` here, but it's not really faster
            // and the complexity doesn't seem worth it.
            let mut dst = [0; SHARED_STR_INLINE_CAPACITY];

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

#[derive(Clone, PartialEq)]
pub(crate) struct OwnedSource {
    // NOTE: We'll want to capture these as better types when backtraces are stable
    debug: String,
    display: String,
    #[cfg(feature = "std")]
    source: Option<SharedContainer<OwnedSource>>,
}

#[cfg(not(feature = "std"))]
impl OwnedSource {
    pub(crate) fn empty() -> Self {
        OwnedSource {
            debug: String::new(),
            display: String::new(),
        }
    }
}

impl<'a> From<stream::Source<'a>> for OwnedSource {
    fn from(err: stream::Source<'a>) -> OwnedSource {
        #[cfg(feature = "std")]
        {
            OwnedSource::collect(err.as_ref())
        }

        #[cfg(not(feature = "std"))]
        {
            let _ = err;
            OwnedSource::empty()
        }
    }
}

impl<'a> From<&'a OwnedSource> for stream::Source<'a> {
    fn from(err: &'a OwnedSource) -> stream::Source<'a> {
        #[cfg(feature = "std")]
        {
            stream::Source::new(err)
        }

        #[cfg(not(feature = "std"))]
        {
            let _ = err;
            stream::Source::empty()
        }
    }
}

impl fmt::Debug for OwnedSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.debug, f)
    }
}

impl fmt::Display for OwnedSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.display, f)
    }
}

#[derive(Clone)]
pub(crate) struct Token {
    #[allow(dead_code)]
    pub(crate) depth: usize,
    pub(crate) kind: TokenKind,
}

// Embedded within a `Token`, which will be shared
#[derive(Clone)]
pub(crate) enum TokenKind {
    MapBegin(stream::MapMeta),
    MapKey,
    MapValue,
    MapEnd,
    SeqBegin(stream::SeqMeta),
    SeqElem,
    SeqEnd,
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    BigSigned(i128),
    BigUnsigned(u128),
    Bool(bool),
    Str(StringContainer<OwnedContainer<str>>),
    Char(char),
    Error(OwnedContainer<OwnedSource>),
    None,
}

impl Token {
    fn stream_owned(&self, mut stream: value::Stream) -> value::Result {
        use self::TokenKind::*;

        match self.kind {
            Signed(v) => stream.i64(v)?,
            Unsigned(v) => stream.u64(v)?,
            Float(v) => stream.f64(v)?,
            BigSigned(v) => stream.i128(v)?,
            BigUnsigned(v) => stream.u128(v)?,
            Bool(v) => stream.bool(v)?,
            Str(ref v) => stream.for_owned().str(&*v)?,
            Char(v) => stream.char(v)?,
            Error(ref v) => stream::Source::from(&**v).stream(stream.for_owned())?,
            None => stream.none()?,
            MapBegin(len) => stream.map_begin(len)?,
            MapKey => {
                stream.map_key_begin()?;
            }
            MapValue => {
                stream.map_value_begin()?;
            }
            MapEnd => stream.map_end()?,
            SeqBegin(len) => stream.seq_begin(len)?,
            SeqElem => {
                stream.seq_elem_begin()?;
            }
            SeqEnd => stream.seq_end()?,
        }

        Ok(())
    }
}

pub(crate) struct TokenBuf {
    depth: usize,
    tokens: Vec<Token>,
}

impl TokenBuf {
    pub(crate) fn new() -> TokenBuf {
        TokenBuf {
            depth: 0,
            tokens: Vec::new(),
        }
    }

    fn collect(v: impl Value) -> Result<Vec<Token>, crate::Error> {
        let mut buf = TokenBuf::new();
        crate::stream_owned(&mut buf, &v)?;

        Ok(buf.tokens)
    }

    fn push(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            depth: self.depth,
            kind,
        });
    }
}

#[cfg(feature = "serde1")]
impl TokenBuf {
    pub(crate) fn clear(&mut self) {
        self.tokens.clear();
        self.depth = 0;
    }

    pub(crate) fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub(crate) fn is_streamable(&self) -> bool {
        self.depth == 0
    }
}

impl<'v> Stream<'v> for TokenBuf {
    fn fmt(&mut self, f: stream::Arguments) -> stream::Result {
        self.push(TokenKind::Str(StringContainer::from(f.to_string())));

        Ok(())
    }

    fn fmt_borrowed(&mut self, f: stream::Arguments<'v>) -> stream::Result {
        self.fmt(f)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.push(TokenKind::Error(OwnedContainer::from(OwnedSource::from(v))));

        Ok(())
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.error(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.push(TokenKind::Signed(v));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.push(TokenKind::Unsigned(v));

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.push(TokenKind::BigSigned(v));

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.push(TokenKind::BigUnsigned(v));

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.push(TokenKind::Float(v));

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.push(TokenKind::Bool(v));

        Ok(())
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.push(TokenKind::Char(v));

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.push(TokenKind::Str(StringContainer::from(v)));

        Ok(())
    }

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.str(v)
    }

    fn none(&mut self) -> stream::Result {
        self.push(TokenKind::None);

        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.depth += 1;

        self.push(TokenKind::MapBegin(len));

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
        self.push(TokenKind::MapKey);

        Ok(())
    }

    fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
        self.map_key()?;
        k.stream(self)
    }

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        self.push(TokenKind::MapValue);

        Ok(())
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.map_value()?;
        v.stream(self)
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        self.push(TokenKind::MapEnd);

        self.depth -= 1;

        Ok(())
    }

    fn seq_begin(&mut self, len: Option(usize)) -> stream::Result {
        self.depth += 1;

        self.push(TokenKind::SeqBegin(len));

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
        self.push(TokenKind::SeqElem);

        Ok(())
    }

    fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
        self.seq_elem()?;
        v.stream(self)
    }

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        self.push(TokenKind::SeqEnd);

        self.depth -= 1;

        Ok(())
    }
}

// Not embedded within a `Token`
#[derive(Clone)]
pub(crate) enum Primitive {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    BigSigned(i128),
    BigUnsigned(u128),
    Bool(bool),
    Str(StringContainer<SharedContainer<str>>),
    Char(char),
    Error(SharedContainer<OwnedSource>),
    None,
}

impl Primitive {
    #[cfg(any(test, feature = "test"))]
    fn to_token(&self) -> Token {
        Token {
            depth: 0,
            kind: match *self {
                Primitive::Signed(v) => TokenKind::Signed(v),
                Primitive::Unsigned(v) => TokenKind::Unsigned(v),
                Primitive::Float(v) => TokenKind::Float(v),
                Primitive::BigSigned(v) => TokenKind::BigSigned(v),
                Primitive::BigUnsigned(v) => TokenKind::BigUnsigned(v),
                Primitive::Bool(v) => TokenKind::Bool(v),
                Primitive::Str(ref v) => TokenKind::Str((&**v).into()),
                Primitive::Char(v) => TokenKind::Char(v),
                Primitive::Error(ref v) => TokenKind::Error((&**v).clone().into()),
                Primitive::None => TokenKind::None,
            },
        }
    }
}

impl Value for Primitive {
    fn stream<'s, 'v>(&'v self, mut stream: value::Stream<'s, 'v>) -> value::Result {
        use self::Primitive::*;

        match *self {
            Signed(v) => stream.i64(v)?,
            Unsigned(v) => stream.u64(v)?,
            Float(v) => stream.f64(v)?,
            BigSigned(v) => stream.i128(v)?,
            BigUnsigned(v) => stream.u128(v)?,
            Bool(v) => stream.bool(v)?,
            Str(ref v) => stream.for_owned().str(&*v)?,
            Char(v) => stream.char(v)?,
            Error(ref v) => stream::Source::from(&**v).stream(stream.for_owned())?,
            None => stream.none()?,
        }

        Ok(())
    }
}

struct PrimitiveBuf {
    primitive: Option<Primitive>,
}

impl PrimitiveBuf {
    fn new() -> PrimitiveBuf {
        PrimitiveBuf { primitive: None }
    }

    fn collect(v: impl Value) -> Option<Primitive> {
        let mut buf = PrimitiveBuf::new();
        crate::stream_owned(&mut buf, &v).ok()?;

        buf.primitive
    }

    fn set(&mut self, primitive: Primitive) {
        self.primitive = Some(primitive);
    }
}

impl<'v> Stream<'v> for PrimitiveBuf {
    fn fmt(&mut self, f: stream::Arguments) -> stream::Result {
        self.set(Primitive::Str(StringContainer::from(f.to_string())));

        Ok(())
    }

    fn fmt_borrowed(&mut self, f: stream::Arguments<'v>) -> stream::Result {
        self.fmt(f)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.set(Primitive::Error(SharedContainer::from(OwnedSource::from(
            v,
        ))));

        Ok(())
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.error(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.set(Primitive::Signed(v));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.set(Primitive::Unsigned(v));

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.set(Primitive::BigSigned(v));

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.set(Primitive::BigUnsigned(v));

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.set(Primitive::Float(v));

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.set(Primitive::Bool(v));

        Ok(())
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.set(Primitive::Char(v));

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.set(Primitive::Str(StringContainer::from(v)));

        Ok(())
    }

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.str(v)
    }

    fn none(&mut self) -> stream::Result {
        self.set(Primitive::None);

        Ok(())
    }

    fn map_begin(&mut self, _: stream::MapMeta) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_key(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_key_collect(&mut self, _: stream::Value) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_value_collect(&mut self, _: stream::Value) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_begin(&mut self, _: stream::SeqMeta) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_elem(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_elem_collect(&mut self, _: stream::Value) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }
}

#[cfg(any(test, feature = "test"))]
impl OwnedValue {
    pub(crate) fn tokens(
        &self,
    ) -> Result<impl crate::std::ops::Deref<Target = [Token]>, crate::Error> {
        match &self.0 {
            ValueInner::Primitive(ref primitive) => Ok(vec![primitive.to_token()].into()),
            ValueInner::Stream(tokens) => Ok(tokens.clone()),
            ValueInner::Error(err) => Err(crate::Error::custom(err)),
            #[cfg(feature = "std")]
            ValueInner::Shared(_) => Err(crate::Error::msg("expected a set of tokens")),
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error::Error;

    impl OwnedSource {
        pub(crate) fn collect(err: &dyn Error) -> Self {
            OwnedSource {
                debug: format!("{:?}", err),
                display: format!("{}", err),
                source: err
                    .source()
                    .map(|source| SharedContainer::from(OwnedSource::collect(source))),
            }
        }
    }

    impl Error for OwnedSource {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source
                .as_ref()
                .map(|source| &**source as &(dyn Error + 'static))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    use crate::{
        std::mem,
        test::{
            self,
            Token,
        },
    };

    struct Map;

    impl Value for Map {
        fn stream<'s, 'v>(&'v self, mut stream: value::Stream<'s, 'v>) -> value::Result {
            let mut stream = stream.for_owned();

            stream.map_begin(Some(2))?;

            stream.map_key(&1)?;
            stream.map_value(&11)?;

            stream.map_key(&2)?;
            stream.map_value(&22)?;

            stream.map_end()
        }
    }

    struct Seq;

    impl Value for Seq {
        fn stream<'s, 'v>(&self, mut stream: value::Stream<'s, 'v>) -> value::Result {
            let mut stream = stream.for_owned();

            stream.seq_begin(Some(2))?;

            stream.seq_elem(&1)?;
            stream.seq_elem(&2)?;

            stream.seq_end()
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn owned_value_size() {
        let size = mem::size_of::<OwnedValue>();
        let limit = {
            #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
            {
                mem::size_of::<u64>() * 6
            }

            #[cfg(not(all(target_arch = "aarch64", target_os = "macos")))]
            {
                mem::size_of::<u64>() * 5
            }
        };

        if size > limit {
            panic!(
                "`OwnedValue` size ({} bytes) is too large (expected up to {} bytes)\n`Primitive`: {} bytes\n`TokenKind`: {} bytes",
                size,
                limit,
                mem::size_of::<Primitive>(),
                mem::size_of::<TokenKind>(),
            );
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn owned_value_is_send_sync() {
        fn is_send_sync<T: Send + Sync>() {}

        is_send_sync::<OwnedValue>();
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn owned_primitive() {
        assert_eq!(
            vec![Token::Str("a format 1".into())],
            test::tokens(&format_args!("a format {}", 1))
        );

        assert_eq!(
            vec![Token::Str("a string".into())],
            test::tokens(&"a string")
        );

        assert_eq!(vec![Token::Unsigned(42u64)], test::tokens(&42u64));

        assert_eq!(vec![Token::Signed(42i64)], test::tokens(&42i64));

        assert_eq!(vec![Token::BigUnsigned(42u128)], test::tokens(&42u128));

        assert_eq!(vec![Token::BigSigned(42i128)], test::tokens(&42i128));

        assert_eq!(vec![Token::Float(42f64)], test::tokens(&42f64));

        assert_eq!(vec![Token::Bool(true)], test::tokens(&true));

        assert_eq!(vec![Token::Char('a')], test::tokens(&'a'));

        assert_eq!(vec![Token::None], test::tokens(&Option::None::<()>));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn owned_map() {
        let v = test::tokens(&Map);

        assert_eq!(
            vec![
                Token::MapBegin(Some(2)),
                Token::Signed(1),
                Token::Signed(11),
                Token::Signed(2),
                Token::Signed(22),
                Token::MapEnd,
            ],
            v
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn owned_seq() {
        let v = test::tokens(&Seq);

        assert_eq!(
            vec![
                Token::SeqBegin(Some(2)),
                Token::Signed(1),
                Token::Signed(2),
                Token::SeqEnd,
            ],
            v
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn inline_str_small() {
        let strs = vec!["", "a", "1234567890123456789012"];

        for s in strs {
            let inline = InlineString::<Box<str>>::from(s);

            assert!(matches!(&inline.0, InlineStringInner::Inline(_, _)));
            assert_eq!(s, &*inline);
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn inline_str_large() {
        let strs = vec!["😎😎😎😎😎😎😎😎", "12345678901234567890123"];

        for s in strs {
            let inline = InlineString::<Box<str>>::from(s);

            assert!(matches!(&inline.0, InlineStringInner::Shared(_)));
            assert_eq!(s, &*inline);
        }
    }

    #[cfg(not(feature = "std"))]
    mod alloc_support {
        use super::*;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn owned_error() {
            let v = test::tokens(stream::Source::empty());

            assert_eq!(vec![Token::None,], v);
        }
    }

    #[cfg(feature = "std")]
    mod std_support {
        use super::*;

        use crate::std::{
            error,
            fmt,
            io,
        };

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn owned_error() {
            #[derive(Debug)]
            struct TestError {
                id: usize,
                source: io::Error,
            }

            impl error::Error for TestError {
                fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                    Some(&self.source)
                }
            }

            impl fmt::Display for TestError {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "it broke!")
                }
            }

            let err = TestError {
                id: 42,
                source: io::Error::from(io::ErrorKind::Other),
            };

            let v = test::tokens(stream::Source::new(&err));

            assert_eq!(vec![Token::Error(test::Source::new(&err)),], v);
        }
    }
}
