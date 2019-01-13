#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[macro_use]
extern crate glium;
extern crate image;
extern crate regex;

pub mod engine;
pub mod game;
pub mod graphics;
pub mod io;
pub mod math;
pub mod parser;

pub trait Printable {
    fn print(&self);
}
