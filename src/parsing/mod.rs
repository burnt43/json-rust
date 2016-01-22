pub mod string;
pub mod number;
pub mod object;

#[derive(Debug)]
enum ParseError {
    UnexpectedToken(char),
    UnterminatedToken(char),
    UnexpectedEndOfInput,
    EmptyStringGiven,
    InvalidUnicodeChar(u32),
}
