pub enum ParseError {
    UnknownToken(String),
    GeneralError(String)
}

pub trait FromFile {
    type ParseResult;

    fn from_file(file: &str) -> Self::ParseResult;
}