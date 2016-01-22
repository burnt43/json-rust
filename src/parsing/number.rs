use parsing::{ParseError};
use types::{Number};

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

fn parse(json_string: &str) -> Result<Number,ParseError> {
    let mut parser: NumberParser = NumberParser::new();
    for ch in json_string.chars() {
        try!(parser.push_token(ch));
    }
    parser.get_result()
}

impl NumberParser {
    fn new() -> NumberParser {
        NumberParser {
            state:  ParseState::SquareOne,
            buffer: String::new(),
        }
    }
    fn get_result(&self) -> Result<Number,ParseError> {
        match self.state {
            ParseState::SquareOne 
            | ParseState::NegativeFound 
            | ParseState::DecimalFound 
            | ParseState::ExponentiationFound 
            | ParseState::SignedExponentiationFound => {
                Err(ParseError::EmptyStringGiven)
            }, //TODO use a real error
            ParseState::FirstDigitZero
            | ParseState::DigitsLeftOfDecimal 
            | ParseState::DigitsRightOfDecimal 
            | ParseState::ExponentiationDigitFound => {
                Ok(self.buffer.parse::<Number>().unwrap())
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
    assert_eq!( parse("0").unwrap(), 0 as Number );
    assert_eq!( parse("0.1").unwrap(), 0.1 as Number );
    assert_eq!( parse("-32").unwrap(), -32 as Number );
    assert_eq!( parse("4.5e1").unwrap(), 45 as Number );
    assert_eq!( parse("3E2").unwrap(), 300 as Number );
    assert_eq!( parse("5e-2").unwrap(), 0.05 as Number );
    assert_eq!( parse("6E-1").unwrap(), 0.6 as Number );
}

#[test]
fn invalid_json_numbers_fail() {
    assert!( parse("").is_err() );
    assert!( parse("--").is_err() );
    assert!( parse("0.0.0").is_err() );
    assert!( parse("0.").is_err() );
    assert!( parse("1.2e1.0").is_err() );
}
