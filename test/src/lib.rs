pub fn assert_tokens<'sval>(value: &'sval (impl sval::Value + ?Sized), tokens: &[Token<'sval>]) {
    let mut stream = Stream(Vec::new());

    value.stream(&mut stream).expect("infallible stream");

    assert_eq!(tokens, &stream.0);
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    Bool(bool),
    Null,
    Tag(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    TextBegin(Option<usize>),
    TextFragment(&'a str),
    TextFragmentComputed(String),
    TextEnd,
    BinaryBegin(Option<usize>),
    BinaryFragment(&'a [u8]),
    BinaryFragmentComputed(Vec<u8>),
    BinaryEnd,
    MapBegin(Option<usize>),
    MapKeyBegin,
    MapKeyEnd,
    MapValueBegin,
    MapValueEnd,
    MapEnd,
    SeqBegin(Option<usize>),
    SeqValueBegin,
    SeqValueEnd,
    SeqEnd,
    EnumBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    EnumEnd(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    TaggedBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    TaggedEnd(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    RecordBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
        Option<usize>,
    ),
    RecordValueBegin(Option<sval::Tag>, sval::Label<'static>),
    RecordValueEnd(Option<sval::Tag>, sval::Label<'static>),
    RecordEnd(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
    ),
    TupleBegin(
        Option<sval::Tag>,
        Option<sval::Label<'static>>,
        Option<sval::Index>,
        Option<usize>,
    ),
    TupleValueBegin(Option<sval::Tag>, sval::Index),
    TupleValueEnd(Option<sval::Tag>, sval::Index),
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
