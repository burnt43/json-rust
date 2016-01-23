use parsing::{Parser, ParseError};
use parsing::nil::{NilParser};
use types::{Object};

enum ParseState {
    SquareOne,
}

struct ObjectParser {
    object:     Object,
    state:      ParseState,
    sub_parser: Box<Parser>,
}

fn parse(json_string: &str) -> Result<Object,ParseError> {
    let mut parser: ObjectParser = ObjectParser::new();
    for ch in json_string.chars() {
        try!(parser.push_token(ch));
    }
    Ok(Object::new())
}

impl ObjectParser {
    fn new() -> ObjectParser {
        ObjectParser {
            object:     Object::new(),
            state:      ParseState::SquareOne,
            sub_parser: Box::new(NilParser::new()),
        }
    }
}

impl Parser for ObjectParser {
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
