use glium;

pub struct Game {
    display: glium::Display,
    is_running: bool
}

impl Game {
    pub fn new(events_loop: &glium::glutin::EventsLoop) -> Game {
         use glium::{glutin};

        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).expect("Failed to create glium display!");

        Game {
            display: display,
            is_running: true
        }
    }

    pub fn run_frame(self, events_loop: &mut glium::glutin::EventsLoop) -> Game {
        use glium::glutin;

        let mut running = self.is_running;

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    _ => ()
                },
                _ => (),
            }
        });

        Game {
            is_running: running,
            display: self.display
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn get_display<'a>(&'a self) -> &'a glium::Display {
        &self.display
    }

    pub fn get_target(&self) -> glium::Frame {
        self.display.draw()
    }
}