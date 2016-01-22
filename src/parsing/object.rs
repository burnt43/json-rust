use parsing::{Parser, ParseError};
use parsing::nil::{NilParser};
use types::{Object};

enum ParseState {
    SquareOne,
}

struct ObjectParser<'a> {
    object:     Object,
    state:      ParseState,
    sub_parser: &'a Parser,
}

fn parse(json_string: &str) -> Result<Object,ParseError> {
    let nil_parser: NilParser = NilParser::new();
    let mut parser: ObjectParser = ObjectParser::new(&nil_parser);
    for ch in json_string.chars() {
        try!(parser.push_token(ch));
    }
    Ok(Object::new())
}

impl<'a> ObjectParser<'a> {
    fn new(nil_parser: &'a NilParser) -> ObjectParser<'a> {
        ObjectParser {
            object:     Object::new(),
            state:      ParseState::SquareOne,
            sub_parser: nil_parser,
        }
    }
}

impl<'a> Parser for ObjectParser<'a> {
    fn push_token(&mut self, ch: char) -> Result<(),ParseError> {
        match self.state {
            ParseState::SquareOne => {
                match ch {
                    '{' => {},
                    _   => { return Err(ParseError::UnexpectedToken(ch)); },
                }
            },
        }
        Ok(())
    }
}

#[test]
fn invalid_objects_fail() {
    assert!( parse("[").is_err() );
}
