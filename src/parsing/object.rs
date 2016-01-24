use parsing::{Parser, ParseError};
use parsing::string::{StringParser};
use parsing::nil::{NilParser};
use types::{Object, Value};

enum ParseState {
    SquareOne,
    ExpectingStringKeyName,
}

struct ObjectParser {
    object:       Object,
    state:        ParseState,
    sub_parser:   Box<Parser>,
    current_pair: (Option<String>,Option<Value>),
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
            object:       Object::new(),
            state:        ParseState::SquareOne,
            sub_parser:   Box::new(NilParser::new()),
            current_pair: (None,None),
        }
    }
}

impl Parser for ObjectParser {
    fn get_result(&self) -> Result<Value, ParseError> {
        Ok(Value::Object(Object::new()))
    }
    fn push_token(&mut self, ch: char) -> Result<(),ParseError> {
        match self.state {
            ParseState::SquareOne => {
                match ch {
                    '{' => {
                        self.state      = ParseState::ExpectingStringKeyName;
                        self.sub_parser = Box::new(StringParser::new());
                    },
                    _   => { 
                        return Err(ParseError::UnexpectedToken(ch));
                    },
                }
            },
            ParseState::ExpectingStringKeyName => {
                match ch {
                    ':' => {
                        match self.sub_parser.push_token(ch) {
                            Ok(_) => {},
                            Err(_) => {
                                //self.current_pair.0 = self.sub_parser.get_result(); //TODO fix
                            },
                        }
                    },
                    _ => {
                        try!(self.sub_parser.push_token(ch));
                    },
                }
            },
        }
        Ok(())
    }
}

#[test]
fn invalid_objects_fail() {
    assert!( parse("[").is_err() );
    assert!( parse("{a").is_err() );
}
