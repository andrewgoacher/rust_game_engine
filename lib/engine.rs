//! A module that contains generic engine code
use glium::{
    glutin::{ContextBuilder, EventsLoop, WindowBuilder},
    Display,
};

use game::Game;

/// Create a glium display and attaches an EventLoop to it
/// # Arguments
/// 
/// `events_loop` - The events loop
/// `title` - the window title
/// 
/// # Remarks
/// 
/// This does not create a borderless or fullscreen window.
/// 
/// # Example
/// 
/// ```rust,no_run
/// let display = create_engine(&events_loop, "Window Title");
/// ```
pub fn create_engine(events_loop: &EventsLoop, title: &str) -> Display {
    let window = WindowBuilder::new().with_title(title);
    let context = ContextBuilder::new().with_depth_buffer(24);

    Display::new(window, context, &events_loop).expect("Failed to create glium display!")
}

/// Runs the main game loop
/// This will run until the events loop recieves a close request.
/// 
/// # Example
/// 
/// ```rust,no_run
/// run(&display, &mut events_loop, game);
/// ```
pub fn run<T: Game>(display: &Display, events_loop: &mut EventsLoop, game: T) {
    let mut running = true;
    let mut game = game;

    use glium::{glutin, Surface};

    while running {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        game = game.on_frame(&mut target);

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                _ => (),
            },
            _ => (),
        });
        target.finish().unwrap();
    }
}
