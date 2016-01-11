use std::collections::{HashMap};

trait ToJson {
    fn to_json(&self) -> String;
}

struct Person {
    age: u8,
}

impl ToJson for Person {
    fn to_json(&self) -> String {
        let mut result: Object = Object::new();
        result.insert("age".to_string(),Value::Number(self.age as Number));
        result.to_json()
    }
}

impl Person {
    fn new(age: u8) -> Person {
        Person { age: age }
    }
}

enum Value {
    Array(Array),
    Boolean(bool),
    None,
    Number(Number),
    Object(Object),
    String(String),
}

type Number = f64;
type Array  = Vec<Value>;
type Object = HashMap<String,Value>;

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

#[derive(Debug)]
enum ParseError {
    UnexpectedToken(char),
    EmptyStringGiven,
    InvalidUnicodeChar(u32),
}

trait SinkOrNoSink {
    fn is_sink(&self) -> bool;
}

enum ParseStringState {
    SquareOne,
    ExpectingChars,
    EscapeCharFound,
    ExpectingEndOfString,
    HexDigitExpected(u8),
}

impl SinkOrNoSink for ParseStringState {
    fn is_sink(&self) -> bool {
        match *self {
            ParseStringState::SquareOne            => false,
            ParseStringState::ExpectingChars       => false,
            ParseStringState::EscapeCharFound      => false,
            ParseStringState::HexDigitExpected(_)  => false,
            ParseStringState::ExpectingEndOfString => true,
        }
    }
}

fn parse_string(json_string: &str) -> Result<String,ParseError> {
    let mut result:     String           = String::new();
    let mut state:      ParseStringState = ParseStringState::SquareOne;
    let mut hex_string: String           = String::new();

    for ch in json_string.chars() {
        match state {
            ParseStringState::SquareOne => {
                match ch {
                    '"' => {
                        state = ParseStringState::ExpectingChars;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch))
                    },
                }
            },
            ParseStringState::ExpectingChars => {
                match ch {
                    '"' => {
                        state = ParseStringState::ExpectingEndOfString;
                    },
                    '\\' => {
                        state = ParseStringState::EscapeCharFound;
                    },
                    _ => {
                        result.push(ch);
                    },
                }
            },
            ParseStringState::EscapeCharFound => {
                match ch {
                    '"' => {
                        result.push('"');
                    },
                    '\\' => {
                        result.push('\\');
                    },
                    '/' => {
                        result.push('/');
                    },
                    'b' => {
                        result.push('\u{08}');
                    },
                    'f' => {
                        result.push('\u{0c}');
                    },
                    'n' => {
                        result.push('\n');
                    },
                    'r' => {
                        result.push('\r');
                    },
                    't' => {
                        result.push('\t');
                    },
                    'u' => {
                        hex_string = String::new();
                        state = ParseStringState::HexDigitExpected(0);
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseStringState::HexDigitExpected(ref mut n @ 0...3) => {
                match ch {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => {
                        hex_string.push(ch);
                        *n+=1;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseStringState::HexDigitExpected(4) => {
                let hex_string_int: u32 = u32::from_str_radix(&hex_string,16).unwrap();
                match std::char::from_u32(hex_string_int) {
                    Some(hex_ch) => {
                        result.push(hex_ch);
                    },
                    None => {
                        return Err(ParseError::InvalidUnicodeChar(hex_string_int));
                    },
                }
            },
            ParseStringState::HexDigitExpected(_) => {
                return Err(ParseError::UnexpectedToken(ch));
            },
            ParseStringState::ExpectingEndOfString => {
                return Err(ParseError::UnexpectedToken(ch));
            },
        }
    }

    Ok(result)
}

enum ParseNumberState {
    SquareOne,
    FirstDigitZero,
    NegativeFound,
    DigitsLeftOfDecimal,
    DecimalFound,
    DigitsRightOfDecimal,
    ExponentiationFound,
    SignedExponentiationFound,
    ExponentiationDigitFound,
}


impl SinkOrNoSink for ParseNumberState {
    fn is_sink(&self) -> bool {
        match *self {
            ParseNumberState::SquareOne                 => false,
            ParseNumberState::FirstDigitZero            => true,
            ParseNumberState::NegativeFound             => false,
            ParseNumberState::DigitsLeftOfDecimal       => true,
            ParseNumberState::DecimalFound              => false,
            ParseNumberState::DigitsRightOfDecimal      => true,
            ParseNumberState::ExponentiationFound       => false,
            ParseNumberState::SignedExponentiationFound => false,
            ParseNumberState::ExponentiationDigitFound  => true,
        }
    }
}

fn parse_number(json_string: &str) -> Result<Number,ParseError> {
    let mut state:         ParseNumberState      = ParseNumberState::SquareOne;
    let mut number_string: String                = String::new();

    // Create the FSA for parsing
    for ch in json_string.chars() {
        match state {
            ParseNumberState::SquareOne => {
                match ch {
                    '-' => {
                        state = ParseNumberState::NegativeFound;
                    },
                    '0' => {
                        state = ParseNumberState::FirstDigitZero;
                    },
                    '1'...'9' => {
                        state = ParseNumberState::DigitsLeftOfDecimal;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    }
                }
            },
            ParseNumberState::DigitsLeftOfDecimal => {
                match ch {
                    '.' => {
                        state = ParseNumberState::DecimalFound;
                    },
                    'e' | 'E' => {
                        state = ParseNumberState::ExponentiationFound;
                    },
                    '0'...'9' => {
                        state = ParseNumberState::DigitsLeftOfDecimal;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    }
                }
            },
            ParseNumberState::FirstDigitZero => {
                match ch {
                    '.' => {
                        state = ParseNumberState::DecimalFound;
                    },
                    'e' | 'E' => {
                        state = ParseNumberState::ExponentiationFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    }
                }
            },
            ParseNumberState::DecimalFound => {
                match ch {
                    '0'...'9' => {
                        state = ParseNumberState::DigitsRightOfDecimal;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseNumberState::DigitsRightOfDecimal => {
                match ch {
                    '0'...'9' => {
                        state = ParseNumberState::DigitsRightOfDecimal;
                    },
                    'e' | 'E' => {
                        state = ParseNumberState::ExponentiationFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseNumberState::NegativeFound => {
                match ch {
                    '0' => {
                        state = ParseNumberState::FirstDigitZero;
                    },
                    '1'...'9' => {
                        state = ParseNumberState::DigitsLeftOfDecimal;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseNumberState::ExponentiationFound => {
                match ch {
                    '-' | '+' => {
                        state = ParseNumberState::SignedExponentiationFound;
                    },
                    '0'...'9' => {
                        state = ParseNumberState::ExponentiationDigitFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseNumberState::SignedExponentiationFound => {
                match ch {
                    '0'...'9' => {
                        state = ParseNumberState::ExponentiationDigitFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseNumberState::ExponentiationDigitFound => {
                match ch {
                    '0'...'9' => {
                        state = ParseNumberState::ExponentiationDigitFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
        }
    }

    if !state.is_sink() {
        match json_string.chars().last() {
            Some(ch) => Err(ParseError::UnexpectedToken(ch)),
            None     => Err(ParseError::EmptyStringGiven),
        }
    } else {
        Ok(json_string.parse::<Number>().unwrap())
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

impl ToJson for Object {
    fn to_json(&self) -> String {
        let mut result: String = String::new();
        result.push_str("{");
        result.push_str( &self.iter().map(|(key,value)| format!("{}:{}",Value::String(key.clone()).to_json(),value.to_json())).collect::<Vec<String>>().join(","));
        result.push_str("}");
        result
    }
}

#[test]
fn valid_json_numbers_pass() {
    assert_eq!( parse_number("0").unwrap(), 0 as Number );
}

#[test]
fn invalid_json_numbers_fail() {
    assert!( parse_number("--").is_err() );
    assert!( parse_number("0.0.0").is_err() );
    assert!( parse_number("0.").is_err() );
}

#[test]
// seems rust can parse any valid JSON number
fn rust_parse_tests() {
    "0".parse::<Number>().unwrap();
    "-0".parse::<Number>().unwrap();
    "0.1".parse::<Number>().unwrap();
    "0.1e3".parse::<Number>().unwrap();
    "0.1e+3".parse::<Number>().unwrap();
    "0.1e-3".parse::<Number>().unwrap();
    "0.1E3".parse::<Number>().unwrap();
    "0.1E+3".parse::<Number>().unwrap();
    "0.1E-3".parse::<Number>().unwrap();
    assert_eq!( u64::from_str_radix("0f",16).unwrap(), 15 );
}

#[test]
fn parse_an_empty_string() {
    assert_eq!(&parse_string("\"\"").unwrap(),"");
}

#[test]
fn parse_a_non_empty_string() {
    assert_eq!(&parse_string("\"foobar\"").unwrap(),"foobar");
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
fn struct_as_object_check() {
    let test_person: Person = Person::new(28);
    assert_eq!(&test_person.to_json(),"{\"age\":28}");
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
