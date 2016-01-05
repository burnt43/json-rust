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

impl ToJson for Value {
    fn to_json(&self) -> String {
        match *self {
            Value::Number(x) => format!("{}",x),
            _ => format!("{}",666),
        }
    }
}

impl ToJson for Array {
    fn to_json(&self) -> String {
        let mut result: String = String::new();
        result.push_str("[");
        result.push_str( &self.iter().map(|value| value.to_json()).collect::<Vec<String>>().join(",") );
        result.push_str("]");
        result
    }
}

#[test]
fn to_json_sanity_check() {
    let some_array: Array = vec![Value::Number(1f64),Value::Number(2f64)];
    assert_eq!(some_array.to_json(),"[1,2]".to_string());
}
