//TODO rewrite this like the StringParser. Get ride of SinkOrNoSink

use parsing::{ParseError, SinkOrNoSink};
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


impl SinkOrNoSink for ParseState {
    fn is_sink(&self) -> bool {
        match *self {
            ParseState::SquareOne                 => false,
            ParseState::FirstDigitZero            => true,
            ParseState::NegativeFound             => false,
            ParseState::DigitsLeftOfDecimal       => true,
            ParseState::DecimalFound              => false,
            ParseState::DigitsRightOfDecimal      => true,
            ParseState::ExponentiationFound       => false,
            ParseState::SignedExponentiationFound => false,
            ParseState::ExponentiationDigitFound  => true,
        }
    }
}

struct NumberParser {
    state:  ParseState,
    buffer: String,
}

fn parse(json_string: &str) -> Result<Number,ParseError> {
    let mut parser: NumberParser = NumberParser::new();
    for ch in json_string.chars() {
        parser.push_token(ch);
    }
    Ok(0f64)
    //Ok(json_string.parse::<Number>().unwrap())
}

impl NumberParser {
    fn new() -> NumberParser {
        NumberParser {
            state:  ParseState::SquareOne,
            buffer: String::new(),
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
}

#[test]
fn invalid_json_numbers_fail() {
    assert!( parse("--").is_err() );
    assert!( parse("0.0.0").is_err() );
    assert!( parse("0.").is_err() );
}
