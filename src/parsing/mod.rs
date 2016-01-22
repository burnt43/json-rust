pub mod string;
pub mod number;
pub mod object;
pub mod nil;

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
}
