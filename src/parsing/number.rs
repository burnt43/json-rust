use parsing::{Parser, ParseError, FromJson};
use types::{Number, Value};

enum ParseState {
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

struct NumberParser {
    state:  ParseState,
    buffer: String,
}

impl FromJson for Number {
    fn from_json(json_string: &str) -> Result<Box<Number>, ParseError> {
        let mut parser: NumberParser = NumberParser::new();
        for ch in json_string.chars() {
            try!(parser.push_token(ch));
        }
        match try!(parser.get_result()) {
            Value::Number(n) => {
                Ok(Box::new(n))
            },
            _ => {
                Err(ParseError::EmptyStringGiven) //TODO use better errors
            }
        }
    }
}

impl NumberParser {
    fn new() -> NumberParser {
        NumberParser {
            state:  ParseState::SquareOne,
            buffer: String::new(),
        }
    }
}

impl Parser for NumberParser {
    fn get_result(&self) -> Result<Value,ParseError> {
        match self.state {
            ParseState::SquareOne => { Err(ParseError::EmptyStringGiven) }, 
            ParseState::NegativeFound 
            | ParseState::DecimalFound 
            | ParseState::ExponentiationFound 
            | ParseState::SignedExponentiationFound => {
                Err(ParseError::UnexpectedToken(self.buffer.chars().last().unwrap()))
            },
            ParseState::FirstDigitZero
            | ParseState::DigitsLeftOfDecimal 
            | ParseState::DigitsRightOfDecimal 
            | ParseState::ExponentiationDigitFound => {
                Ok(Value::Number(self.buffer.parse::<Number>().unwrap()))
            },
        }
    }
    fn push_token(&mut self, ch: char) -> Result<(),ParseError> {
        match self.state {
            ParseState::SquareOne => {
                match ch {
                    '-' => {
                        self.state = ParseState::NegativeFound;
                    },
                    '0' => {
                        self.state = ParseState::FirstDigitZero;
                    },
                    '1'...'9' => {
                        self.state = ParseState::DigitsLeftOfDecimal;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    }
                }
            },
            ParseState::DigitsLeftOfDecimal => {
                match ch {
                    '.' => {
                        self.state = ParseState::DecimalFound;
                    },
                    'e' | 'E' => {
                        self.state = ParseState::ExponentiationFound;
                    },
                    '0'...'9' => {
                        self.state = ParseState::DigitsLeftOfDecimal;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    }
                }
            },
            ParseState::FirstDigitZero => {
                match ch {
                    '.' => {
                        self.state = ParseState::DecimalFound;
                    },
                    'e' | 'E' => {
                        self.state = ParseState::ExponentiationFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    }
                }
            },
            ParseState::DecimalFound => {
                match ch {
                    '0'...'9' => {
                        self.state = ParseState::DigitsRightOfDecimal;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::DigitsRightOfDecimal => {
                match ch {
                    '0'...'9' => {
                        self.state = ParseState::DigitsRightOfDecimal;
                    },
                    'e' | 'E' => {
                        self.state = ParseState::ExponentiationFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::NegativeFound => {
                match ch {
                    '0' => {
                        self.state = ParseState::FirstDigitZero;
                    },
                    '1'...'9' => {
                        self.state = ParseState::DigitsLeftOfDecimal;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::ExponentiationFound => {
                match ch {
                    '-' | '+' => {
                        self.state = ParseState::SignedExponentiationFound;
                    },
                    '0'...'9' => {
                        self.state = ParseState::ExponentiationDigitFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::SignedExponentiationFound => {
                match ch {
                    '0'...'9' => {
                        self.state = ParseState::ExponentiationDigitFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::ExponentiationDigitFound => {
                match ch {
                    '0'...'9' => {
                        self.state = ParseState::ExponentiationDigitFound;
                    },
                    _ => {
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
        }
        self.buffer.push(ch);
        Ok(())
    }
}


#[test]
fn valid_json_numbers_pass() {
    assert_eq!( *Number::from_json("0").unwrap(), 0 as Number );
    assert_eq!( *Number::from_json("0.1").unwrap(), 0.1 as Number );
    assert_eq!( *Number::from_json("-32").unwrap(), -32 as Number );
    assert_eq!( *Number::from_json("4.5e1").unwrap(), 45 as Number );
    assert_eq!( *Number::from_json("3E2").unwrap(), 300 as Number );
    assert_eq!( *Number::from_json("5e-2").unwrap(), 0.05 as Number );
    assert_eq!( *Number::from_json("6E-1").unwrap(), 0.6 as Number );
    assert_eq!( *Number::from_json("3e+3").unwrap(), 3000 as Number );
}

#[test]
fn invalid_json_numbers_fail() {
    assert!( Number::from_json("").is_err() );
    assert!( Number::from_json("--").is_err() );
    assert!( Number::from_json("0.0.0").is_err() );
    assert!( Number::from_json("0.").is_err() );
    assert!( Number::from_json("1.2e1.0").is_err() );
}
