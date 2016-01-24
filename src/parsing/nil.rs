use parsing::{Parser, ParseError};
use types::{Value};

fn parse(string: &str) -> Result<(),ParseError> {
    let mut parser: NilParser = NilParser::new();
    for ch in string.chars() {
        try!(parser.push_token(ch))
    }
    Err(ParseError::EmptyStringGiven)
}

pub struct NilParser;

impl NilParser {
    pub fn new() -> NilParser {
        NilParser
    }
}

impl Parser for NilParser {
    fn push_token(&mut self, ch: char) -> Result<(),ParseError> {
        Err(ParseError::UnexpectedToken(ch))
    }
    fn get_result(&self) -> Result<Value, ParseError> {
        Err(ParseError::EmptyStringGiven) //TODO make these errors better
    }
}

#[test]
fn nil_parser_fails_everything() {
    assert!(parse("").is_err());
}
