use parsing::{ParseError, SinkOrNoSink};
use types::{Number};

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

fn parse(json_string: &str) -> Result<Number,ParseError> {
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
