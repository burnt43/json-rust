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

trait SinkOrNoSink {
    fn is_sink(&self) -> bool;
}
