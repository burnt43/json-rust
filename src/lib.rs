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
            Value::Number(n) => format!("{}",n),
            Value::String(ref s) => format!("\"{}\"",s),
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
fn array_of_numbers_check() {
    let some_array: Array = vec![Value::Number(1f64),Value::Number(2f64)];
    assert_eq!(some_array.to_json(),"[1,2]".to_string());
}

#[test]
fn array_of_strings_check() {
    let some_array: Array = vec![Value::String("a".to_string()),Value::String("b".to_string())];
    assert_eq!(some_array.to_json(),"[\"a\",\"b\"]".to_string());
}
