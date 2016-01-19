use std::collections::{HashMap};

pub type Number = f64;
pub type Array  = Vec<Value>;
pub type Object = HashMap<String,Value>;

pub enum Value {
    Array(Array),
    Boolean(bool),
    None,
    Number(Number),
    Object(Object),
    String(String),
}
