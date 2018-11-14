use std::collections::BTreeMap;

use serde_value::Id as SerdeId;
use val_value::Id;

fn main() {
    // A map that implements `val::value::Value`
    let mut map = BTreeMap::new();

    map.insert(Id::new(1), vec!["Hello", "World"]);
    map.insert(Id::new(2), vec!["World", "Hello"]);

    let json = serde_json::to_string(&val_serde::to_serialize(map)).unwrap();
    println!("{}", json);

    // A map that implements `serde::Serialize`
    let mut map = BTreeMap::new();

    map.insert(SerdeId::new(1), vec!["Hello", "World"]);
    map.insert(SerdeId::new(2), vec!["World", "Hello"]);

    let json = serde_json::to_string(&map).unwrap();
    println!("{}", json);
}
