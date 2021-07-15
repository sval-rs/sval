#[cfg(feature = "std")]
use crate::std::sync::Arc;

pub(crate) type OwnedContainer<T> = Box<T>;

#[cfg(feature = "std")]
pub(crate) type SharedContainer<T> = Arc<T>;
#[cfg(not(feature = "std"))]
pub(crate) type SharedContainer<T> = OwnedContainer<T>;

pub(crate) type StringContainer<T> = InlineString<T>;

mod source;
mod str;
mod ident;
mod tag;

mod value;
mod stream;

pub use self::{
    source::OwnedSource,
    ident::OwnedIdent,
    tag::OwnedTag,
    value::OwnedValue,
    stream::OwnedStream,
};

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
            let mut stream = stream.owned();

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
            let mut stream = stream.owned();

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
