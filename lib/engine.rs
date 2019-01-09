use glium::{
    glutin::{WindowBuilder,ContextBuilder,EventsLoop},
    Display
};

use game::Game;

pub fn create_engine(events_loop: &EventsLoop, title: &str) -> Display {
    let window = WindowBuilder::new()
        .with_title(title);
    let context = ContextBuilder::new()
        .with_depth_buffer(24);

    Display::new(window, context, &events_loop)
        .expect("Failed to create glium display!")    
}

pub fn run<T: Game>(display: &Display, events_loop: &mut EventsLoop, game: T) {
    let mut running = true;
    let mut game = game;

    use glium::{glutin,Surface};

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