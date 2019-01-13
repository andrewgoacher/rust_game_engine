//! This module contains a set of types and traits that represent a game
//! The methods here are used within a game loop and control the flow of the 
//! game.
use glium::Frame;

/// A collection of methods that a game should implement 
/// to be run from within an engine.
pub trait Game {
    /// Method to be run once per frame
    /// 
    /// # Arguments
    /// 
    /// `self` - the instance of game
    /// `frame` - (mutable reference) - The frame object that the frame renders to
    fn on_frame(self, frame: &mut Frame) -> Self;
}
