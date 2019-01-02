use glium;

pub trait Game {
    fn on_frame(self, frame: &mut glium::Frame, engine: &Engine) -> Self;
}

pub struct Engine {
    display: glium::Display,
    is_running: bool,
}

impl Engine {
    pub fn new(events_loop: &glium::glutin::EventsLoop) -> Engine {
        use glium::glutin;

        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop)
            .expect("Failed to create glium display!");

        Engine {
            display: display,
            is_running: true,
        }
    }

    pub fn run<T: Game>(self, events_loop: &mut glium::glutin::EventsLoop, game: T) -> Engine {
        use glium::{glutin, Surface};
        let mut running = self.is_running();
        let mut game = game;

        while running {
            let mut target = self.display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

            game = game.on_frame(&mut target, &self);

            events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    _ => (),
                },
                _ => (),
            });
            target.finish().unwrap();
        }

        Engine { 
            is_running: running,
            ..self
         }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn get_display<'a>(&'a self) -> &'a glium::Display {
        &self.display
    }
}
