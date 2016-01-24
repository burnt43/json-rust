pub mod string;
pub mod number;
pub mod object;
pub mod nil;
use types::{Value};

#[derive(Debug)]
enum ParseError {
    UnexpectedToken(char),
    UnterminatedToken(char),
    UnexpectedEndOfInput,
    EmptyStringGiven,
    InvalidUnicodeChar(u32),
}

trait Parser {
    fn push_token(&mut self, ch: char) -> Result<(),ParseError>;
    fn get_result(&self) -> Result<Value, ParseError>;
}
