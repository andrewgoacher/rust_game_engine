use glium::Frame;

pub trait Game {
    fn on_frame(self, frame: &mut Frame) -> Self;
}
