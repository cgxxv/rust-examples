#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_value;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_value::Value;

#[derive(Serialize)]
struct Justmike2000 {
    a: String,
    b: String,
    c: u64,
}

fn get_field_by_name<T, R>(data: T, field: &str) -> R
where
    T: Serialize,
    R: DeserializeOwned,
{
    let mut map = match serde_value::to_value(data) {
        Ok(Value::Map(map)) => map,
        _ => panic!("expected a struct"),
    };

    let key = Value::String(field.to_owned());
    let value = match map.remove(&key) {
        Some(value) => value,
        None => panic!("no such field"),
    };

    match R::deserialize(value) {
        Ok(r) => r,
        Err(_) => panic!("wrong type?"),
    }
}

fn main() {
    let j = Justmike2000 {
        a: "string A".to_owned(),
        b: "string B".to_owned(),
        c: 1,
    };

    let a: String = get_field_by_name(&j, "a");
    println!("a = {:?}", a);

    let b: String = get_field_by_name(&j, "b");
    println!("b = {:?}", b);

    let c: u64 = get_field_by_name(&j, "c");
    println!("c = {:?}", c);
}
