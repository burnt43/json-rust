use std::char;
use parsing::{Parser,ParseError, FromJson};
use types::{Value};

pub struct StringParser {
    buffer:     String,
    hex_string: String,
    state:      ParseState,
}

enum ParseState {
    SquareOne,
    ExpectingChars,
    EscapeCharFound,
    ExpectingEndOfString,
    HexDigitExpected(u8),
}

impl FromJson for String {
    fn from_json(json_string: &str) -> Result<Box<String>,ParseError> {
        let mut parser: StringParser = StringParser::new();
        for ch in json_string.chars() {
            try!(parser.push_token(ch))
        }
        match try!(parser.get_result()) {
            Value::String(s) => {
                Ok(Box::new(s))
            },
            _ => {
                Err(ParseError::EmptyStringGiven) //TODO use better errors
            }
        }
    }
}

impl StringParser {
    pub fn new() -> StringParser {
        StringParser{
            buffer:     String::new(),
            hex_string: String::new(),
            state:      ParseState::SquareOne
        }
    }
}

impl Parser for StringParser {
    fn get_result(&self) -> Result<Value, ParseError> {
        match self.state {
            ParseState::SquareOne            => { Err(ParseError::EmptyStringGiven) },
            ParseState::ExpectingChars       => { Err(ParseError::UnterminatedToken('"')) },
            ParseState::EscapeCharFound      => { Err(ParseError::UnterminatedToken('"')) },
            ParseState::HexDigitExpected(_)  => { Err(ParseError::UnterminatedToken('"')) },
            ParseState::ExpectingEndOfString => { Ok(Value::String(self.buffer.clone())) },
        }
    }
    fn push_token(&mut self, ch: char) -> Result<(),ParseError> {
        match self.state {
            ParseState::SquareOne => {
                match ch {
                    '"' => {
                        self.state = ParseState::ExpectingChars;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch))
                    },
                }
            },
            ParseState::ExpectingChars => {
                match ch {
                    '"' => {
                        self.state = ParseState::ExpectingEndOfString;
                    },
                    '\\' => {
                        self.state = ParseState::EscapeCharFound;
                    },
                    // TODO add the control characters that JSON does not allow here
                    _ => {
                        self.buffer.push(ch);
                    },
                }
            },
            ParseState::EscapeCharFound => {
                match ch {
                    '"' => {
                        self.buffer.push('"');
                        self.state = ParseState::ExpectingChars;
                    },
                    '\\' => {
                        self.buffer.push('\\');
                        self.state = ParseState::ExpectingChars;
                    },
                    '/' => {
                        self.buffer.push('/');
                        self.state = ParseState::ExpectingChars;
                    },
                    'b' => {
                        self.buffer.push('\u{08}');
                        self.state = ParseState::ExpectingChars;
                    },
                    'f' => {
                        self.buffer.push('\u{0c}');
                        self.state = ParseState::ExpectingChars;
                    },
                    'n' => {
                        self.buffer.push('\n');
                        self.state = ParseState::ExpectingChars;
                    },
                    'r' => {
                        self.buffer.push('\r');
                        self.state = ParseState::ExpectingChars;
                    },
                    't' => {
                        self.buffer.push('\t');
                        self.state = ParseState::ExpectingChars;
                    },
                    'u' => {
                        self.hex_string = String::new();
                        self.state = ParseState::HexDigitExpected(0);
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::HexDigitExpected(ref mut n @ 0...2) => {
                match ch {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => {
                        self.hex_string.push(ch);
                        *n+=1; // effectively changes the state
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::HexDigitExpected(3) => {
                match ch {
                    '0'...'9' | 'a'...'f' | 'A'...'F' => {
                        self.hex_string.push(ch);
                        let hex_string_int: u32 = u32::from_str_radix(&self.hex_string,16).unwrap();
                        match char::from_u32(hex_string_int) {
                            Some(hex_ch) => {
                                self.buffer.push(hex_ch);
                                self.state = ParseState::ExpectingChars;
                            },
                            None => {
                                return Err(ParseError::InvalidUnicodeChar(hex_string_int));
                            },
                        }
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::HexDigitExpected(_) => {
                return Err(ParseError::UnexpectedToken(ch));
            },
            ParseState::ExpectingEndOfString => {
                return Err(ParseError::UnexpectedToken(ch));
            },
        }
        Ok(())
    }
}

// HAPPY PATHS
#[test]
fn parse_an_empty_string() {
    assert_eq!(&*String::from_json("\"\"").unwrap(),"");
}

#[test]
fn parse_a_non_empty_string() {
    assert_eq!(&*String::from_json("\"foobar\"").unwrap(),"foobar");
}

#[test]
fn parse_strings_with_escapes() {
    assert_eq!(&*String::from_json("\"\\n\"").unwrap(),"\n");
    assert_eq!(&*String::from_json("\"\\u0041\"").unwrap(),"A");
}

// SAD PATHS
#[test]
fn parse_unterminated_string_fails() {
    assert!(String::from_json("\"unterminated string").is_err());
}

#[test]
fn parse_nothingness_fails() {
    assert!(String::from_json("").is_err());
}

#[test]
fn parse_invalid_escape_sequence_fails() {
    assert!(String::from_json("\\h").is_err());
}
