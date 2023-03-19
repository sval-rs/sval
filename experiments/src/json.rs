mod from_slice;

pub use self::from_slice::*;

#[cfg(test)]
mod tests {
    use super::*;

    use sval_derive::*;

    #[derive(Value)]
    struct MapStruct {
        field_0: i32,
        field_1: bool,
        field_2: &'static str,
    }

    #[test]
    fn json_slice_roundtrip() {
        let json = sval_json::stream_to_string(MapStruct {
            field_0: 42,
            field_1: true,
            field_2: "abc",
        })
        .unwrap();

        let slice = from_slice(&json);

        assert_eq!(json, sval_json::stream_to_string(slice).unwrap());
    }
}
