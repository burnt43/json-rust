use parsing::{Parser, ParseError};

fn parse(string: &str) -> Result<(),ParseError> {
    let mut parser: NilParser = NilParser::new();
    for ch in string.chars() {
        try!(parser.push_token(ch))
    }
    Err(ParseError::EmptyStringGiven)
}

pub struct NilParser;

impl NilParser {
    fn new() -> NilParser {
        NilParser
    }
}

impl Parser for NilParser {
    fn push_token(&mut self, ch: char) -> Result<(),ParseError> {
        Err(ParseError::UnexpectedToken(ch))
    }
}

#[test]
fn nil_parser_fails_everything() {
    assert!(parse("").is_err());
}
