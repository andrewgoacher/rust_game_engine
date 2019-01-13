//! This module contains a set of types and traits that represent a game
//! The methods here are used within a game loop and control the flow of the 
//! game.
use glium::Frame;

/// A collection of methods that a game should implement 
/// to be run from within an engine.
/// 
/// # Example
/// 
/// ```rust
/// struct GameExample {
///     total_frames: u32
/// }
/// 
/// impl Game for GameExample {
///     fn on_frame(self, frame: &mut glium::Frame) -> GameExample {
///         GameExample {
///             total_frames: self.total_frames + 1
///         }
///     }
/// }
/// ```
pub trait Game {
    /// Method to be run once per frame
    /// 
    /// # Arguments
    /// 
    /// `self` - the instance of game
    /// `frame` - (mutable reference) - The frame object that the frame renders to
    fn on_frame(self, frame: &mut Frame) -> Self;
}
