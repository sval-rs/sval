/*!
Test utilities for `sval`.
*/

use std::any::type_name;

/**
Assert that a value streams to exactly the sequence of tokens provided.
*/
#[track_caller]
pub fn assert_tokens<'sval, V: sval::Value + ?Sized>(value: &'sval V, tokens: &[Token<'sval>]) {
    let mut stream = TokenBuf::new();

    match value.stream(&mut stream) {
        Ok(()) => {
            assert_eq!(
                tokens,
                stream.as_tokens(),
                "{} != {}",
                sval_fmt::stream_to_string(AsValue(tokens)),
                sval_fmt::stream_to_string(AsValue(stream.as_tokens()))
            );

            #[cfg(test)]
            {
                let mut dyn_stream = &mut TokenBuf::new();

                value
                    .stream(&mut dyn_stream as &mut dyn sval_dynamic::Stream<'sval>)
                    .unwrap();

                assert_eq!(
                    tokens,
                    dyn_stream.as_tokens(),
                    "(dyn) {} != {}",
                    sval_fmt::stream_to_string(AsValue(tokens)),
                    sval_fmt::stream_to_string(AsValue(dyn_stream.as_tokens()))
                );
            }
        }
        Err(_) => stream.fail::<V>(),
    }
}

/**
Assert that a value streams without failing.
*/
#[track_caller]
pub fn assert_valid<V: sval::Value>(value: V) {
    let mut stream = TokenBuf::new();

    if let Err(_) = value.stream(&mut stream) {
        stream.fail::<V>();
    }
}

/**
Assert that a value fails to stream.
*/
#[track_caller]
pub fn assert_invalid<V: sval::Value>(value: V) {
    let mut stream = TokenBuf::new();

    if let Ok(_) = value.stream(&mut stream) {
        panic!(
            "expected streaming `{}` to fail, but it produced `{}`",
            type_name::<V>(),
            sval_fmt::stream_to_string(AsValue(&stream.tokens))
        )
    }
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
    [`sval::Stream::tag_hint`]
    */
    TagHint(sval::Tag),
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
    /**
    [`sval::Stream::record_tuple_begin`].
     */
    RecordTupleBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
        Option<usize>,
    ),
    /**
    [`sval::Stream::record_tuple_value_begin`].
     */
    RecordTupleValueBegin(Option<sval::Tag>, sval::Label<'static>, sval::Index),
    /**
    [`sval::Stream::record_tuple_value_end`].
     */
    RecordTupleValueEnd(Option<sval::Tag>, sval::Label<'static>, sval::Index),
    /**
    [`sval::Stream::record_tuple_end`].
     */
    RecordTupleEnd(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
}

// Avoid exposing `sval_buffer`-like functionality here
// Use `sval_buffer` instead
struct AsValue<'a, 'b>(&'a [Token<'b>]);

impl<'a, 'b> sval::Value for AsValue<'a, 'b> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        for token in self.0 {
            match token {
                Token::U8(v) => stream.u8(*v)?,
                Token::U16(v) => stream.u16(*v)?,
                Token::U32(v) => stream.u32(*v)?,
                Token::U64(v) => stream.u64(*v)?,
                Token::U128(v) => stream.u128(*v)?,
                Token::I8(v) => stream.i8(*v)?,
                Token::I16(v) => stream.i16(*v)?,
                Token::I32(v) => stream.i32(*v)?,
                Token::I64(v) => stream.i64(*v)?,
                Token::I128(v) => stream.i128(*v)?,
                Token::F32(v) => stream.f32(*v)?,
                Token::F64(v) => stream.f64(*v)?,
                Token::Bool(v) => stream.bool(*v)?,
                Token::Null => stream.null()?,
                Token::Tag(tag, label, index) => {
                    stream.tag(tag.as_ref(), label.as_ref(), index.as_ref())?
                }
                Token::TagHint(tag) => {
                    stream.tag_hint(tag)?;
                }
                Token::TextBegin(num_bytes) => stream.text_begin(*num_bytes)?,
                Token::TextFragment(v) => stream.text_fragment(*v)?,
                Token::TextFragmentComputed(v) => stream.text_fragment_computed(&**v)?,
                Token::TextEnd => stream.text_end()?,
                Token::BinaryBegin(num_bytes) => stream.binary_begin(*num_bytes)?,
                Token::BinaryFragment(v) => stream.binary_fragment(*v)?,
                Token::BinaryFragmentComputed(v) => stream.binary_fragment_computed(&**v)?,
                Token::BinaryEnd => stream.binary_end()?,
                Token::MapBegin(num_entries) => stream.map_begin(*num_entries)?,
                Token::MapKeyBegin => stream.map_key_begin()?,
                Token::MapKeyEnd => stream.map_key_end()?,
                Token::MapValueBegin => stream.map_value_begin()?,
                Token::MapValueEnd => stream.map_value_end()?,
                Token::MapEnd => stream.map_end()?,
                Token::SeqBegin(num_entries) => stream.seq_begin(*num_entries)?,
                Token::SeqValueBegin => stream.seq_value_begin()?,
                Token::SeqValueEnd => stream.seq_value_end()?,
                Token::SeqEnd => stream.seq_end()?,
                Token::EnumBegin(tag, label, index) => {
                    stream.enum_begin(tag.as_ref(), label.as_ref(), index.as_ref())?
                }
                Token::EnumEnd(tag, label, index) => {
                    stream.enum_end(tag.as_ref(), label.as_ref(), index.as_ref())?
                }
                Token::TaggedBegin(tag, label, index) => {
                    stream.tagged_begin(tag.as_ref(), label.as_ref(), index.as_ref())?
                }
                Token::TaggedEnd(tag, label, index) => {
                    stream.tagged_end(tag.as_ref(), label.as_ref(), index.as_ref())?
                }
                Token::RecordBegin(tag, label, index, num_entries) => stream.record_begin(
                    tag.as_ref(),
                    label.as_ref(),
                    index.as_ref(),
                    *num_entries,
                )?,
                Token::RecordValueBegin(tag, label) => {
                    stream.record_value_begin(tag.as_ref(), label)?
                }
                Token::RecordValueEnd(tag, label) => {
                    stream.record_value_end(tag.as_ref(), label)?
                }
                Token::RecordEnd(tag, label, index) => {
                    stream.record_end(tag.as_ref(), label.as_ref(), index.as_ref())?
                }
                Token::TupleBegin(tag, label, index, num_entries) => stream.tuple_begin(
                    tag.as_ref(),
                    label.as_ref(),
                    index.as_ref(),
                    *num_entries,
                )?,
                Token::TupleValueBegin(tag, index) => {
                    stream.tuple_value_begin(tag.as_ref(), index)?
                }
                Token::TupleValueEnd(tag, index) => stream.tuple_value_end(tag.as_ref(), index)?,
                Token::TupleEnd(tag, label, index) => {
                    stream.tuple_end(tag.as_ref(), label.as_ref(), index.as_ref())?
                }
                Token::RecordTupleBegin(tag, label, index, num_entries) => stream
                    .record_tuple_begin(
                        tag.as_ref(),
                        label.as_ref(),
                        index.as_ref(),
                        *num_entries,
                    )?,
                Token::RecordTupleValueBegin(tag, label, index) => {
                    stream.record_tuple_value_begin(tag.as_ref(), label, index)?
                }
                Token::RecordTupleValueEnd(tag, label, index) => {
                    stream.record_tuple_value_end(tag.as_ref(), label, index)?
                }
                Token::RecordTupleEnd(tag, label, index) => {
                    stream.record_tuple_end(tag.as_ref(), label.as_ref(), index.as_ref())?
                }
            }
        }

        Ok(())
    }
}

/**
A buffer for collecting test tokens.

This type shouldn't be used as a general-purpose buffer.
See the `sval-buffer` library for that.
*/
#[derive(Default, PartialEq, Debug)]
pub struct TokenBuf<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> TokenBuf<'a> {
    /**
    Create a new, empty token buffer.
    */
    pub fn new() -> Self {
        TokenBuf { tokens: Vec::new() }
    }

    /**
    Get the underlying tokens in this buffer.
    */
    pub fn as_tokens(&self) -> &[Token<'a>] {
        &self.tokens
    }

    fn push(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    #[track_caller]
    fn fail<T: ?Sized>(&self) {
        panic!(
            "the `impl sval::Value for {}` is invalid\nstreamed to:\n  `{}`\nraw:\n  `{:?}`",
            type_name::<T>(),
            sval_fmt::stream_to_string(AsValue(&self.tokens)),
            self.tokens
        );
    }
}

impl<'sval> sval::Stream<'sval> for TokenBuf<'sval> {
    fn null(&mut self) -> sval::Result {
        self.push(Token::Null);
        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.push(Token::Bool(value));
        Ok(())
    }

    fn text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.push(Token::TextBegin(num_bytes));
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

    fn binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.push(Token::BinaryBegin(num_bytes));
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

    fn tag_hint(&mut self, tag: &sval::Tag) -> sval::Result {
        self.push(Token::TagHint(tag.clone()));

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

    fn record_tuple_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.push(Token::RecordTupleBegin(
            tag.cloned(),
            label.map(|label| label.to_owned()),
            index.cloned(),
            num_entries,
        ));
        Ok(())
    }

    fn record_tuple_value_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        index: &sval::Index,
    ) -> sval::Result {
        self.push(Token::RecordTupleValueBegin(
            tag.cloned(),
            label.to_owned(),
            index.clone(),
        ));
        Ok(())
    }

    fn record_tuple_value_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        index: &sval::Index,
    ) -> sval::Result {
        self.push(Token::RecordTupleValueEnd(
            tag.cloned(),
            label.to_owned(),
            index.clone(),
        ));
        Ok(())
    }

    fn record_tuple_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.push(Token::RecordTupleEnd(
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

    use std::{
        collections::{BTreeMap, HashMap},
        fmt,
    };

    fn assert_tokens<'sval>(value: &'sval impl sval::Value, tokens: &[Token<'sval>]) {
        super::assert_tokens(value, tokens);
        super::assert_tokens(value as &'sval dyn sval_dynamic::Value, tokens);
    }

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
            &sval::BinarySlice::new(&[1, 2, 3]),
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
        assert_tokens(&(&[] as &[u8]), &[Token::SeqBegin(Some(0)), Token::SeqEnd]);

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
            &(&[1] as &[i32]),
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

        assert_tokens(
            &(1, 2, 3),
            &[
                Token::TupleBegin(None, None, None, Some(3)),
                Token::TupleValueBegin(None, sval::Index::new(0)),
                Token::I32(1),
                Token::TupleValueEnd(None, sval::Index::new(0)),
                Token::TupleValueBegin(None, sval::Index::new(1)),
                Token::I32(2),
                Token::TupleValueEnd(None, sval::Index::new(1)),
                Token::TupleValueBegin(None, sval::Index::new(2)),
                Token::I32(3),
                Token::TupleValueEnd(None, sval::Index::new(2)),
                Token::TupleEnd(None, None, None),
            ],
        );
    }

    #[test]
    fn stream_tagged() {
        struct BigInt {
            is_negative: bool,
            digits: &'static str,
        }

        impl fmt::Display for BigInt {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.is_negative {
                    f.write_str("-")?;
                }

                f.write_str(self.digits)
            }
        }

        impl sval::Value for BigInt {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                stream.tagged_begin(Some(&sval::tags::NUMBER), None, None)?;
                sval::stream_display(stream, self)?;
                stream.tagged_end(Some(&sval::tags::NUMBER), None, None)
            }
        }

        assert_tokens(
            &BigInt {
                is_negative: true,
                digits: "123456",
            },
            &[
                Token::TaggedBegin(Some(sval::tags::NUMBER), None, None),
                Token::TextBegin(None),
                Token::TextFragmentComputed("-".to_owned()),
                Token::TextFragmentComputed("123456".to_owned()),
                Token::TextEnd,
                Token::TaggedEnd(Some(sval::tags::NUMBER), None, None),
            ],
        );
    }

    #[test]
    fn stream_tag_hints() {
        struct WithHints;

        impl sval::Value for WithHints {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                stream.tag_hint(&sval::Tag::new("test"))?;

                stream.value(&42)?;

                stream.tag_hint(&sval::Tag::new("test"))
            }
        }

        assert_tokens(
            &WithHints,
            &[
                Token::TagHint(sval::Tag::new("test")),
                Token::I32(42),
                Token::TagHint(sval::Tag::new("test")),
            ],
        );
    }

    #[test]
    fn stream_invalid() {
        #[derive(Debug)]
        enum KaboomMaybe {
            Nah,
            Kaboom,
        }

        impl sval::Value for KaboomMaybe {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                match self {
                    KaboomMaybe::Nah => stream.bool(true),
                    KaboomMaybe::Kaboom => sval::error(),
                }
            }
        }

        let map = {
            let mut map = BTreeMap::new();

            map.insert(1, KaboomMaybe::Nah);
            map.insert(2, KaboomMaybe::Nah);
            map.insert(3, KaboomMaybe::Kaboom);
            map.insert(4, KaboomMaybe::Nah);

            map
        };

        let e = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            assert_valid(&map);
        }))
        .unwrap_err();

        assert!(e
            .downcast_ref::<String>()
            .unwrap()
            .contains("{ 1: true, 2: true, 3: "));
    }
}
