//! A small engine framework using glium and opengl
//! 
//! This library is mostly a learning environment for personal use but its aim is
//! to be a performant 2d and 3d engine.
//! # Organisation
//! 
//! This crate contains several modules [`graphics`] contains the majority of the render code, [`math`] contains a set of utilities for Vector and Matrix maths
//! creating the actual display / engine requires use of [`engine`] and creating a game requires implemting the trait `Game` from within the [`game`] module.
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