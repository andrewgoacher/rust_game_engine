#[macro_use]
extern crate glium;
extern crate image;
extern crate regex;

pub mod game;
pub mod graphics;
pub mod io;
pub mod math;
pub mod parser;
pub mod engine;

pub trait Printable {
    fn print(&self);
}