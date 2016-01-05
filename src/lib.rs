use std::collections::{HashMap};

trait ToJson {
    fn to_json(&self) -> String;
}

enum Value {
    String(String),
    Number(f64),
    Object(Object),
    Array(Array),
    Boolean(bool),
    None,
}

type Array  = Vec<Value>;
type Object = HashMap<String,Value>;

impl ToJson for Array {
    fn to_json(&self) -> String {
        return format!("{}",5);
    }
}

#[test]
fn to_json_sanity_check() {
    let some_array: Array = vec![Value::Number(1f64),Value::Number(2f64)];
    assert_eq!(some_array.to_json(),"5".to_string());
}
