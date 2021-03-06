use types::{Value, Array, Object};

trait ToJson {
    fn to_json(&self) -> String;
}

impl ToJson for Value {
    fn to_json(&self) -> String {
        match *self {
            Value::Array(ref x)  => format!("{}",x.to_json()),
            Value::Boolean(x)    => format!("{}",x),
            Value::None          => "null".to_string(),
            Value::Number(x)     => format!("{}",x),
            Value::Object(ref x) => format!("{}",x.to_json()),
            Value::String(ref x) => format!("\"{}\"",x),
        }
    }
}

impl ToJson for Array {
    fn to_json(&self) -> String {
        let mut result: String = String::new();
        result.push_str("[");
        result.push_str( &self
                         .iter()
                         .map(|value| value.to_json()).collect::<Vec<String>>().join(",") );
        result.push_str("]");
        result
    }
}

impl ToJson for Object {
    fn to_json(&self) -> String {
        let mut result: String = String::new();
        result.push_str("{");
        result.push_str( &self
                         .iter()
                         .map(|(key,value)| format!("{}:{}", Value::String(key.clone()).to_json(), value.to_json()))
                         .collect::<Vec<String>>().join(",") );
        result.push_str("}");
        result
    }
}

#[test]
fn empty_object_check() {
    let test_object: Object = Object::new();
    assert_eq!(&test_object.to_json(),"{}");
}

#[test]
fn simple_object_check() {
    let mut test_object: Object = Object::new();
    test_object.insert("name".to_string(),Value::String("James".to_string()));
    assert_eq!(&test_object.to_json(),"{\"name\":\"James\"}");
}

#[test]
fn array_of_numbers_check() {
    let test_array: Array = vec![Value::Number(1f64),Value::Number(2f64)];
    assert_eq!(&test_array.to_json(),"[1,2]");
}

#[test]
fn array_of_strings_check() {
    let test_array: Array = vec![Value::String("a".to_string()),Value::String("b".to_string())];
    assert_eq!(&test_array.to_json(),"[\"a\",\"b\"]");
}

#[test]
fn array_of_booleans_check() {
    let test_array: Array = vec![Value::Boolean(true),Value::Boolean(false)];
    assert_eq!(&test_array.to_json(),"[true,false]");
}

#[test]
fn array_of_empty_arrays_check() {
    let test_array: Array = vec![Value::Array(Array::new()),Value::Array(Array::new())];
    assert_eq!(&test_array.to_json(),"[[],[]]");
}

#[test]
fn array_of_empty_objects_check() {
    let test_array: Array = vec![Value::Object(Object::new()),Value::Object(Object::new())];
    assert_eq!(&test_array.to_json(),"[{},{}]");
}
