//! A helper module for common io patterns and functions
use std::fs::File;
use std::io::{prelude::*, Cursor};

/// This method will return a cursor that has a collection of unsigned bytes.AsMut
/// 
/// # Arguments
/// 
/// `file` - a file to load
pub fn to_cursor(mut file: File) -> Cursor<Vec<u8>> {
    let mut contents = vec![];
    file.read_to_end(&mut contents).ok();
    Cursor::new(contents)
}
