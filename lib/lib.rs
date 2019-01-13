//! A small engine framework using glium and opengl
//! 
//! This library is mostly a learning environment for personal use but its aim is
//! to be a performant 2d and 3d engine.
//! # Organisation
//! 
//! This crate contains several modules [`graphics`] contains the majority of the render code, [`math`] contains a set of utilities for Vector and Matrix maths
//! creating the actual display / engine requires use of [`engine`] and creating a game requires implemting the trait `Game` from within the [`game`] module.
//! 
//! A minimal implementation would be
//! ```
//! rust_game_engine::game::Game;
//! 
//! struct Demo {}
//! impl Game for Demo {
//!     fn on_frame(self, frame: &mut glium::Frame) -> Demo {
//!         Demo {}
//!     }
//! }
//! ```
//! 
//! To run the demo you need to create an events loop and an engine
//! 
//! ```
//! let mut events_loop = glium::glutin::EventsLoop::new();
//! let display = create_engine(&events_loop, "Window Title");
//! let game = Demo {};
//! 
//! ```
//! 
//! The [`engine`] module exposes a method `run` to run the game
//! 
//! ``` 
//! use rust_game_engine::engine::run;
//! 
//! run(&display, &mut events_loop, game);
//! ```
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