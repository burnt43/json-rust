use std::collections::{HashMap};

#[test]
fn it_works() {
}

enum Value {
    String(String),
    Number(f64),
    JsonObject(JsonObject),
    JsonArray(JsonArray),
    Boolean(bool),
    None,
}

type JsonArray  = Vec<Value>;
type JsonObject = HashMap<String,Value>;

#[test]
fn simple_array() {
    let json_string: &str = "[1,2,3,4]";
    let value: Value = Value::JsonArray(vec![Value::Number(1f64),Value::Number(2f64),Value::Number(3f64),Value::Number(4f64)]);
    match value {
        Value::JsonArray(json_array) => {
            match json_array[0] {
                Value::Number(n) => assert_eq!(n,1f64),
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
