use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

pub fn to_cursor(mut file: File) -> Cursor<Vec<u8>> {
    let mut contents = vec![];
    file.read_to_end(&mut contents).ok();
    Cursor::new(contents)
}