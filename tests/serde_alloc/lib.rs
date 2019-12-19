#![cfg(test)]

#[macro_use]
extern crate sval;

#[macro_use]
extern crate serde;

use sval::{
    test::Token as SvalToken,
    value::{
        self,
        Value,
    },
};

use serde_test::{
    assert_ser_tokens,
    Token as SerdeToken,
};

#[derive(Serialize, Value)]
#[sval(derive_from = "serde")]
enum Tagged {
    Unit,
    NewType(i32),
    Tuple(i32, i32),
    Struct { a: i32, b: i32 },
}

#[derive(Value, Serialize)]
struct Struct<'a> {
    a: i32,
    b: i32,
    #[sval(rename = "renamed")]
    c: Nested<'a>,
}

#[derive(Value, Serialize)]
struct Nested<'a> {
    a: i32,
    b: &'a str,
}

struct Anonymous;

impl Value for Anonymous {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        stream.map_key_begin()?.i64(1)?;

        stream.map_value_begin()?.map_begin(None)?;

        stream.map_key(2)?;

        stream.map_value_begin()?.seq_begin(None)?;

        stream.seq_elem_begin()?.i64(3)?;

        stream.seq_end()?;

        stream.map_end()?;

        stream.map_key(11)?;

        stream.map_value(111)?;

        stream.map_end()
    }
}

#[test]
fn sval_derive() {
    use self::SvalToken as Token;

    let v = sval::test::tokens(Struct {
        a: 1,
        b: 2,
        c: Nested { a: 3, b: "Hello!" },
    });
    assert_eq!(
        vec![
            Token::MapBegin(Some(3)),
            Token::Str(String::from("a")),
            Token::Signed(1),
            Token::Str(String::from("b")),
            Token::Signed(2),
            Token::Str(String::from("renamed")),
            Token::MapBegin(Some(2)),
            Token::Str(String::from("a")),
            Token::Signed(3),
            Token::Str(String::from("b")),
            Token::Str(String::from("Hello!")),
            Token::MapEnd,
            Token::MapEnd,
        ],
        v
    );
}

#[test]
fn sval_derive_from_serde() {
    use self::SvalToken as Token;

    let v = sval::test::tokens(Tagged::NewType(1));
    assert_eq!(
        vec![
            Token::MapBegin(Some(1)),
            Token::Str(String::from("NewType")),
            Token::SeqBegin(Some(1)),
            Token::Signed(1),
            Token::SeqEnd,
            Token::MapEnd,
        ],
        v
    );
}

#[test]
fn serde_to_sval_tagged() {
    use self::SvalToken as Token;

    let v = sval::test::tokens(sval::serde::to_value(Tagged::Unit));
    assert_eq!(vec![Token::Str(String::from("Unit"))], v);

    let v = sval::test::tokens(sval::serde::to_value(Tagged::NewType(1)));
    assert_eq!(
        vec![
            Token::MapBegin(Some(1)),
            Token::Str(String::from("NewType")),
            Token::SeqBegin(Some(1)),
            Token::Signed(1),
            Token::SeqEnd,
            Token::MapEnd,
        ],
        v
    );

    let v = sval::test::tokens(sval::serde::to_value(Tagged::Tuple(1, 2)));
    assert_eq!(
        vec![
            Token::MapBegin(Some(1)),
            Token::Str(String::from("Tuple")),
            Token::SeqBegin(Some(2)),
            Token::Signed(1),
            Token::Signed(2),
            Token::SeqEnd,
            Token::MapEnd,
        ],
        v
    );

    let v = sval::test::tokens(sval::serde::to_value(Tagged::Struct { a: 1, b: 2 }));
    assert_eq!(
        vec![
            Token::MapBegin(Some(1)),
            Token::Str(String::from("Struct")),
            Token::MapBegin(Some(2)),
            Token::Str(String::from("a")),
            Token::Signed(1),
            Token::Str(String::from("b")),
            Token::Signed(2),
            Token::MapEnd,
            Token::MapEnd,
        ],
        v
    );
}

#[test]
fn sval_to_serde_anonymous() {
    use self::SerdeToken as Token;

    assert_ser_tokens(
        &sval::serde::to_serialize(Anonymous),
        &[
            Token::Map { len: None },
            Token::I64(1),
            Token::Map { len: None },
            Token::I64(2),
            Token::Seq { len: None },
            Token::I64(3),
            Token::SeqEnd,
            Token::MapEnd,
            Token::I64(11),
            Token::I64(111),
            Token::MapEnd,
        ],
    );
}
