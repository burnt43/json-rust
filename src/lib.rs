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
    UnterminatedToken(char),
    UnexpectedEndOfInput,
    EmptyStringGiven,
    InvalidUnicodeChar(u32),
}

trait SinkOrNoSink {
    fn is_sink(&self) -> bool;
}

#[derive(Debug)]
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

struct StringParser {
    buffer:     String,
    hex_string: String,
    state:      ParseStringState,
}

fn parse_string(json_string: &str) -> Result<String,ParseError> {
    let mut parser: StringParser                    = StringParser::new();
    let mut result: ParserResult<String,ParseError> = ParserResult::Err(ParseError::EmptyStringGiven);
    for ch in json_string.chars() {
        result = parser.push(ch);
        match result {
            ParserResult::Ok(_)      => {},
            ParserResult::Incomplete => {},
            ParserResult::Err(ref e) => { break; }
        }
    }
    match result {
        ParserResult::Incomplete => { Err(ParseError::UnterminatedToken('"')) },
        ParserResult::Ok(string) => { Ok(string) },
        ParserResult::Err(e)     => { Err(e) },
    }
}

enum ParserResult<T,E> {
    Ok(T),
    Err(E),
    Incomplete,
}

impl StringParser {
    fn new() -> StringParser {
        StringParser{
            buffer:     String::new(),
            hex_string: String::new(),
            state:      ParseStringState::SquareOne
        }
    }
    fn push(&mut self, ch: char) -> ParserResult<String,ParseError> {
        match self.state {
            ParseStringState::SquareOne => {
                match ch {
                    '"' => {
                        self.state = ParseStringState::ExpectingChars;
                    },
                    _ => {
                        return ParserResult::Err(ParseError::UnexpectedToken(ch))
                    },
                }
            },
            ParseStringState::ExpectingChars => {
                match ch {
                    '"' => {
                        self.state = ParseStringState::ExpectingEndOfString;
                        return ParserResult::Ok(self.buffer.clone());
                    },
                    '\\' => {
                        self.state = ParseStringState::EscapeCharFound;
                    },
                    // TODO add the control characters that JSON does not allow here
                    _ => {
                        self.buffer.push(ch);
                    },
                }
            },
            ParseStringState::EscapeCharFound => {
                match ch {
                    '"' => {
                        self.buffer.push('"');
                        self.state = ParseStringState::ExpectingChars;
                    },
                    '\\' => {
                        self.buffer.push('\\');
                        self.state = ParseStringState::ExpectingChars;
                    },
                    '/' => {
                        self.buffer.push('/');
                        self.state = ParseStringState::ExpectingChars;
                    },
                    'b' => {
                        self.buffer.push('\u{08}');
                        self.state = ParseStringState::ExpectingChars;
                    },
                    'f' => {
                        self.buffer.push('\u{0c}');
                        self.state = ParseStringState::ExpectingChars;
                    },
                    'n' => {
                        self.buffer.push('\n');
                        self.state = ParseStringState::ExpectingChars;
                    },
                    'r' => {
                        self.buffer.push('\r');
                        self.state = ParseStringState::ExpectingChars;
                    },
                    't' => {
                        self.buffer.push('\t');
                        self.state = ParseStringState::ExpectingChars;
                    },
                    'u' => {
                        self.hex_string = String::new();
                        self.state = ParseStringState::HexDigitExpected(0);
                    },
                    _ => {
                        return ParserResult::Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseStringState::HexDigitExpected(ref mut n @ 0...2) => {
                match ch {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => {
                        self.hex_string.push(ch);
                        *n+=1; // effectively changes the state
                    },
                    _ => {
                        return ParserResult::Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseStringState::HexDigitExpected(3) => {
                match ch {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => {
                        self.hex_string.push(ch);
                        let hex_string_int: u32 = u32::from_str_radix(&self.hex_string,16).unwrap();
                        match std::char::from_u32(hex_string_int) {
                            Some(hex_ch) => {
                                self.buffer.push(hex_ch);
                                self.state = ParseStringState::ExpectingChars;
                            },
                            None => {
                                return ParserResult::Err(ParseError::InvalidUnicodeChar(hex_string_int));
                            },
                        }
                    },
                    _ => {
                        return ParserResult::Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseStringState::HexDigitExpected(_) => {
                return ParserResult::Err(ParseError::UnexpectedToken(ch));
            },
            ParseStringState::ExpectingEndOfString => {
                return ParserResult::Err(ParseError::UnexpectedToken(ch));
            },
        }
        ParserResult::Incomplete
    }
}

enum ParseObjectState {
    SquareOne,
    ExpectingKey,
}

impl SinkOrNoSink for ParseObjectState {
    fn is_sink(&self) -> bool {
        match *self {
            ParseObjectState::SquareOne => false,
            ParseObjectState::ExpectingKey => false,
        }
    }
}

fn parse_object(json_string: &str) -> Result<Object,ParseError> {
    let mut state: ParseObjectState = ParseObjectState::SquareOne;
    let mut current_key: String = String::new();

    for ch in json_string.chars() {
        match state {
            ParseObjectState::SquareOne => {
                match ch {
                    '{' => {
                        state = ParseObjectState::ExpectingKey;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseObjectState::ExpectingKey => {
                match ch {
                    '"' => {
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
        }
    }

    Ok(Object::new())
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
fn parse_strings_with_escapes() {
    assert_eq!(&parse_string("\"\\n\"").unwrap(),"\n");
    assert_eq!(&parse_string("\"\\u0041\"").unwrap(),"A");
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
