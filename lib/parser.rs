#[derive(Debug)]
/// A helper enum to represent parsing errors
pub enum ParseError {
    /// The token being parsed is unrecognised.
    /// The string should represent the token
    UnknownToken(String),
    /// A general error has been encountered that there is no
    /// specific error for.
    GeneralError(String),
}

/// A trait that exposes a parse like behaviour
/// intended 
pub trait FromFile {
    /// Represents the result of the parse 
    /// This should handle both a succesful or unsuccessful parse
    type ParseResult;

    /// a method to handle parsing something from a file provided
    /// 
    /// # Arguments
    /// 
    /// `file` - the path to the file to parse
    /// 
    /// # Remarks
    /// 
    /// It's recommended that when implementing this trait, the ParseResult is 
    /// `Result<T, ParseError>`
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// struct ParsedResult {}
    /// 
    /// impl FromFile for ParsedResult {
    ///     type ParseResult = Result<ParsedResult, ParseError>;
    /// 
    ///     fn from_file(file: &str) -> Self::ParsedResult {
    ///         Ok(ParsedResult)
    ///     }
    /// }
    /// 
    /// match ParsedResult::from_file("Some/path/to.file") {
    ///     Err(e) => match e {
    ///         ParseError::UnknownToken(token) => panic!("Token"),
    ///         ParseError::GeneralError(error) => panic!("Generic Error")
    ///     },
    ///     Ok(result) => result
    /// }
    /// ```
    fn from_file(file: &str) -> Self::ParseResult;
}
