use std::char;
use parsing::{ParseResult, ParseError};

struct StringParser {
    buffer:     String,
    hex_string: String,
    state:      ParseState,
}

#[derive(Debug)]
enum ParseState {
    SquareOne,
    ExpectingChars,
    EscapeCharFound,
    ExpectingEndOfString,
    HexDigitExpected(u8),
}

fn parse(json_string: &str) -> Result<String,ParseError> {
    let mut parser: StringParser        = StringParser::new();
    let mut result: ParseResult<String> = ParseResult::Err(ParseError::EmptyStringGiven);
    for ch in json_string.chars() {
        result = parser.push(ch);
        match result {
            ParseResult::Err(ref e) => { break; },
            _                        => {},
        }
    }
    match result {
        ParseResult::Incomplete => { Err(ParseError::UnterminatedToken('"')) },
        ParseResult::Ok(string) => { Ok(string) },
        ParseResult::Err(e)     => { Err(e) },
    }
}

impl StringParser {
    fn new() -> StringParser {
        StringParser{
            buffer:     String::new(),
            hex_string: String::new(),
            state:      ParseState::SquareOne
        }
    }
    fn push(&mut self, ch: char) -> ParseResult<String> {
        match self.state {
            ParseState::SquareOne => {
                match ch {
                    '"' => {
                        self.state = ParseState::ExpectingChars;
                    },
                    _ => {
                        return ParseResult::Err(ParseError::UnexpectedToken(ch))
                    },
                }
            },
            ParseState::ExpectingChars => {
                match ch {
                    '"' => {
                        self.state = ParseState::ExpectingEndOfString;
                        return ParseResult::Ok(self.buffer.clone());
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
                        return ParseResult::Err(ParseError::UnexpectedToken(ch));
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
                        return ParseResult::Err(ParseError::UnexpectedToken(ch));
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
                                return ParseResult::Err(ParseError::InvalidUnicodeChar(hex_string_int));
                            },
                        }
                    },
                    _ => {
                        return ParseResult::Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::HexDigitExpected(_) => {
                return ParseResult::Err(ParseError::UnexpectedToken(ch));
            },
            ParseState::ExpectingEndOfString => {
                return ParseResult::Err(ParseError::UnexpectedToken(ch));
            },
        }
        ParseResult::Incomplete
    }
}

#[test]
fn parse_an_empty_string() {
    assert_eq!(&parse("\"\"").unwrap(),"");
}

#[test]
fn parse_a_non_empty_string() {
    assert_eq!(&parse("\"foobar\"").unwrap(),"foobar");
}

#[test]
fn parse_strings_with_escapes() {
    assert_eq!(&parse("\"\\n\"").unwrap(),"\n");
    assert_eq!(&parse("\"\\u0041\"").unwrap(),"A");
}
