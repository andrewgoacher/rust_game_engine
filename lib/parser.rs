pub enum ParseError {
    UnknownToken(String),
    GeneralError(String)
}

pub trait Parseable {
    type ParseResult;

    fn from_file(file: &str) -> Self::ParseResult;
}