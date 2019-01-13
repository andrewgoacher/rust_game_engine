//! This module represents functionality for parsing files
//! within this engine there are many types that can be loaded from
//! the file system and the common behaviours and types can be represented here.
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
    fn from_file(file: &str) -> Self::ParseResult;
}
