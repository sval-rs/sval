/*!
Test utilities for `sval`.
*/

/**
Assert that a value streams to exactly the sequence of tokens provided.
*/
pub fn assert_tokens<'sval>(value: &'sval (impl sval::Value + ?Sized), tokens: &[Token<'sval>]) {
    let mut stream = Stream(Vec::new());

    value.stream(&mut stream).expect("infallible stream");

    assert_eq!(tokens, &stream.0);
}

/**
A token representing a specific call to an [`sval::Stream`] method.
*/
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Token<'a> {
    /**
    [`sval::Stream::u8`].
    */
    U8(u8),
    /**
    [`sval::Stream::u16`].
    */
    U16(u16),
    /**
    [`sval::Stream::u32`].
    */
    U32(u32),
    /**
    [`sval::Stream::u64`].
    */
    U64(u64),
    /**
    [`sval::Stream::u128`].
    */
    U128(u128),
    /**
    [`sval::Stream::i8`].
    */
    I8(i8),
    /**
    [`sval::Stream::i16`].
    */
    I16(i16),
    /**
    [`sval::Stream::i32`].
    */
    I32(i32),
    /**
    [`sval::Stream::i64`].
    */
    I64(i64),
    /**
    [`sval::Stream::i128`].
    */
    I128(i128),
    /**
    [`sval::Stream::f32`].
    */
    F32(f32),
    /**
    [`sval::Stream::f64`].
    */
    F64(f64),
    /**
    [`sval::Stream::bool`].
    */
    Bool(bool),
    /**
    [`sval::Stream::null`].
    */
    Null,
    /**
    [`sval::Stream::tag`].
    */
    Tag(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    /**
    [`sval::Stream::text_begin`].
    */
    TextBegin(Option<usize>),
    /**
    [`sval::Stream::text_fragment`].
    */
    TextFragment(&'a str),
    /**
    [`sval::Stream::text_fragment_computed`].
    */
    TextFragmentComputed(String),
    /**
    [`sval::Stream::text_end`].
    */
    TextEnd,
    /**
    [`sval::Stream::binary_begin`].
    */
    BinaryBegin(Option<usize>),
    /**
    [`sval::Stream::binary_fragment`].
    */
    BinaryFragment(&'a [u8]),
    /**
    [`sval::Stream::binary_fragment_computed`].
    */
    BinaryFragmentComputed(Vec<u8>),
    /**
    [`sval::Stream::binary_end`].
    */
    BinaryEnd,
    /**
    [`sval::Stream::map_begin`].
    */
    MapBegin(Option<usize>),
    /**
    [`sval::Stream::map_key_begin`].
    */
    MapKeyBegin,
    /**
    [`sval::Stream::map_key_end`].
    */
    MapKeyEnd,
    /**
    [`sval::Stream::map_value_begin`].
    */
    MapValueBegin,
    /**
    [`sval::Stream::map_value_end`].
    */
    MapValueEnd,
    /**
    [`sval::Stream::map_end`].
    */
    MapEnd,
    /**
    [`sval::Stream::seq_begin`].
    */
    SeqBegin(Option<usize>),
    /**
    [`sval::Stream::seq_value_begin`].
    */
    SeqValueBegin,
    /**
    [`sval::Stream::seq_value_end`].
    */
    SeqValueEnd,
    /**
    [`sval::Stream::seq_end`].
    */
    SeqEnd,
    /**
    [`sval::Stream::enum_begin`].
    */
    EnumBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    /**
    [`sval::Stream::enum_end`].
    */
    EnumEnd(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    /**
    [`sval::Stream::tagged_begin`].
    */
    TaggedBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    /**
    [`sval::Stream::tagged_end`].
    */
    TaggedEnd(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    /**
    [`sval::Stream::record_begin`].
    */
    RecordBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
        Option<usize>,
    ),
    /**
    [`sval::Stream::record_value_begin`].
    */
    RecordValueBegin(Option<sval::Tag>, sval::Label<'static>),
    /**
    [`sval::Stream::record_value_end`].
    */
    RecordValueEnd(Option<sval::Tag>, sval::Label<'static>),
    /**
    [`sval::Stream::record_end`],
    */
    RecordEnd(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    /**
    [`sval::Stream::tuple_begin`].
    */
    TupleBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
        Option<usize>,
    ),
    /**
    [`sval::Stream::tuple_value_begin`].
    */
    TupleValueBegin(Option<sval::Tag>, sval::Index),
    /**
    [`sval::Stream::tuple_value_end`].
    */
    TupleValueEnd(Option<sval::Tag>, sval::Index),
    /**
    [`sval::Stream::tuple_end`].
    */
    TupleEnd(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
}

struct Stream<'a>(Vec<Token<'a>>);

impl<'a> Stream<'a> {
    fn push(&mut self, token: Token<'a>) {
        self.0.push(token);
    }
}

impl<'sval> sval::Stream<'sval> for Stream<'sval> {
    fn null(&mut self) -> sval::Result {
        self.push(Token::Null);
        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.push(Token::Bool(value));
        Ok(())
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.push(Token::TextBegin(num_bytes_hint));
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.push(Token::TextFragmentComputed(fragment.to_owned()));
        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        self.push(Token::TextEnd);
        Ok(())
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.push(Token::I64(value));
        Ok(())
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.push(Token::F64(value));
        Ok(())
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.push(Token::SeqBegin(num_entries_hint));
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.push(Token::SeqValueBegin);
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.push(Token::SeqValueEnd);
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.push(Token::SeqEnd);
        Ok(())
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.push(Token::TextFragment(fragment));
        Ok(())
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.push(Token::BinaryBegin(num_bytes_hint));
        Ok(())
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.push(Token::BinaryFragment(fragment));
        Ok(())
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.push(Token::BinaryFragmentComputed(fragment.to_vec()));
        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        self.push(Token::BinaryEnd);
        Ok(())
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.push(Token::U8(value));
        Ok(())
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.push(Token::U16(value));
        Ok(())
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.push(Token::U32(value));
        Ok(())
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.push(Token::U64(value));
        Ok(())
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.push(Token::U128(value));
        Ok(())
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.push(Token::I8(value));
        Ok(())
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.push(Token::I16(value));
        Ok(())
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.push(Token::I32(value));
        Ok(())
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.push(Token::I128(value));
        Ok(())
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.push(Token::F32(value));
        Ok(())
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.push(Token::MapBegin(num_entries_hint));
        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.push(Token::MapKeyBegin);
        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.push(Token::MapKeyEnd);
        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.push(Token::MapValueBegin);
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.push(Token::MapValueEnd);
        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.push(Token::MapEnd);
        Ok(())
    }

    fn enum_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.push(Token::EnumBegin(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
        ));
        Ok(())
    }

    fn enum_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.push(Token::EnumEnd(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
        ));
        Ok(())
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.push(Token::TaggedBegin(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
        ));
        Ok(())
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.push(Token::TaggedEnd(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
        ));
        Ok(())
    }

    fn tag(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.push(Token::Tag(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
        ));
        Ok(())
    }

    fn record_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.push(Token::RecordBegin(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
            num_entries,
        ));
        Ok(())
    }

    fn record_value_begin(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        self.push(Token::RecordValueBegin(tag.cloned(), label.to_owned()));
        Ok(())
    }

    fn record_value_end(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        self.push(Token::RecordValueEnd(tag.cloned(), label.to_owned()));
        Ok(())
    }

    fn record_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.push(Token::RecordEnd(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
        ));
        Ok(())
    }

    fn tuple_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.push(Token::TupleBegin(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
            num_entries,
        ));
        Ok(())
    }

    fn tuple_value_begin(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
        self.push(Token::TupleValueBegin(tag.cloned(), index.clone()));
        Ok(())
    }

    fn tuple_value_end(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
        self.push(Token::TupleValueEnd(tag.cloned(), index.clone()));
        Ok(())
    }

    fn tuple_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.push(Token::TupleEnd(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
        ));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn stream_primitive() {
        assert_tokens(&1u8, &[Token::U8(1)]);
        assert_tokens(&2u16, &[Token::U16(2)]);
        assert_tokens(&3u32, &[Token::U32(3)]);
        assert_tokens(&4u64, &[Token::U64(4)]);
        assert_tokens(&5u128, &[Token::U128(5)]);

        assert_tokens(&-1i8, &[Token::I8(-1)]);
        assert_tokens(&-2i16, &[Token::I16(-2)]);
        assert_tokens(&-3i32, &[Token::I32(-3)]);
        assert_tokens(&-4i64, &[Token::I64(-4)]);
        assert_tokens(&-5i128, &[Token::I128(-5)]);

        assert_tokens(&3.14f32, &[Token::F32(3.14)]);
        assert_tokens(&3.14159f64, &[Token::F64(3.14159)]);

        assert_tokens(&true, &[Token::Bool(true)]);
    }

    #[test]
    fn stream_option() {
        assert_tokens(
            &Some(1),
            &[
                Token::TaggedBegin(
                    Some(sval::tags::RUST_OPTION_SOME),
                    Some(sval::Label::new("Some")),
                    Some(sval::Index::new(1)),
                ),
                Token::I32(1),
                Token::TaggedEnd(
                    Some(sval::tags::RUST_OPTION_SOME),
                    Some(sval::Label::new("Some")),
                    Some(sval::Index::new(1)),
                ),
            ],
        );

        assert_tokens(
            &None::<i32>,
            &[Token::Tag(
                Some(sval::tags::RUST_OPTION_NONE),
                Some(sval::Label::new("None")),
                Some(sval::Index::new(0)),
            )],
        );
    }

    #[test]
    fn stream_unit() {
        assert_tokens(&(), &[Token::Tag(Some(sval::tags::RUST_UNIT), None, None)]);
    }

    #[test]
    fn stream_binary() {
        assert_tokens(
            sval::BinarySlice::new(&[1, 2, 3]),
            &[
                Token::BinaryBegin(Some(3)),
                Token::BinaryFragment(&[1, 2, 3]),
                Token::BinaryEnd,
            ],
        );

        assert_tokens(
            sval::BinaryArray::new(&[1, 2, 3]),
            &[
                Token::TaggedBegin(Some(sval::tags::CONSTANT_SIZE), None, None),
                Token::BinaryBegin(Some(3)),
                Token::BinaryFragment(&[1, 2, 3]),
                Token::BinaryEnd,
                Token::TaggedEnd(Some(sval::tags::CONSTANT_SIZE), None, None),
            ],
        );
    }

    #[test]
    fn stream_map_empty() {
        assert_tokens(
            &BTreeMap::<u8, u8>::new(),
            &[Token::MapBegin(Some(0)), Token::MapEnd],
        );
        assert_tokens(
            &HashMap::<u8, u8>::new(),
            &[Token::MapBegin(Some(0)), Token::MapEnd],
        );
    }

    #[test]
    fn stream_map() {
        let map = {
            let mut map = BTreeMap::new();

            map.insert(1, 2);

            map
        };
        assert_tokens(
            &map,
            &[
                Token::MapBegin(Some(1)),
                Token::MapKeyBegin,
                Token::I32(1),
                Token::MapKeyEnd,
                Token::MapValueBegin,
                Token::I32(2),
                Token::MapValueEnd,
                Token::MapEnd,
            ],
        );

        let map = {
            let mut map = HashMap::new();

            map.insert(1, 2);

            map
        };
        assert_tokens(
            &map,
            &[
                Token::MapBegin(Some(1)),
                Token::MapKeyBegin,
                Token::I32(1),
                Token::MapKeyEnd,
                Token::MapValueBegin,
                Token::I32(2),
                Token::MapValueEnd,
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn stream_seq_empty() {
        assert_tokens(&[] as &[u8], &[Token::SeqBegin(Some(0)), Token::SeqEnd]);

        assert_tokens(
            &[] as &[u8; 0],
            &[
                Token::TaggedBegin(Some(sval::tags::CONSTANT_SIZE), None, None),
                Token::SeqBegin(Some(0)),
                Token::SeqEnd,
                Token::TaggedEnd(Some(sval::tags::CONSTANT_SIZE), None, None),
            ],
        );

        assert_tokens(
            &Vec::<u8>::new(),
            &[Token::SeqBegin(Some(0)), Token::SeqEnd],
        );
    }

    #[test]
    fn stream_seq() {
        assert_tokens(
            &[1] as &[i32],
            &[
                Token::SeqBegin(Some(1)),
                Token::SeqValueBegin,
                Token::I32(1),
                Token::SeqValueEnd,
                Token::SeqEnd,
            ],
        );

        assert_tokens(
            &[1],
            &[
                Token::TaggedBegin(Some(sval::tags::CONSTANT_SIZE), None, None),
                Token::SeqBegin(Some(1)),
                Token::SeqValueBegin,
                Token::I32(1),
                Token::SeqValueEnd,
                Token::SeqEnd,
                Token::TaggedEnd(Some(sval::tags::CONSTANT_SIZE), None, None),
            ],
        );

        assert_tokens(
            &vec![1],
            &[
                Token::SeqBegin(Some(1)),
                Token::SeqValueBegin,
                Token::I32(1),
                Token::SeqValueEnd,
                Token::SeqEnd,
            ],
        );
    }
}
