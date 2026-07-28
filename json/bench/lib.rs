#[path = "data.rs"]
mod data;

#[test]
fn compat() {
    let serde = serde_json::to_string(&data::input_struct()).unwrap();
    let sval = sval_json::stream_to_string(data::input_struct()).unwrap();

    assert_eq!(serde, sval);
}
