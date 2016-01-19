pub mod string;
pub mod number;

#[derive(Debug)]
enum ParseError {
    UnexpectedToken(char),
    UnterminatedToken(char),
    UnexpectedEndOfInput,
    EmptyStringGiven,
    InvalidUnicodeChar(u32),
}

enum ParseResult<T> {
    Ok(T),
    Err(ParseError),
    Incomplete,
}

trait SinkOrNoSink {
    fn is_sink(&self) -> bool;
}
