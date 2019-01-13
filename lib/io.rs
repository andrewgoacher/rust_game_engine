//! A helper module for common io patterns and functions
use std::fs::File;
use std::io::{prelude::*, Cursor};

/// This method will return a cursor that has a collection of unsigned bytes.AsMut
/// 
/// # Arguments
/// 
/// `file` - a file to load
/// 
/// # Example
/// 
/// ```rust,no_run 
/// let mut file = File::open("path/to.file").unwrap();
/// 
/// let cursor = to_cursor(mut file);
/// ```
pub fn to_cursor(mut file: File) -> Cursor<Vec<u8>> {
    let mut contents = vec![];
    file.read_to_end(&mut contents).ok();
    Cursor::new(contents)
}
