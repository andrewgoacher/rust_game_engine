use glium;
use glium::Program;

use std::fs::read_to_string;
use game::Game;

pub fn create_shader(vertex: &str, fragment: &str, game: &Game) -> Program {
    let vertex_shader_src =
        read_to_string(&vertex).expect(format!("Failed to find {}", &vertex).as_str());
    let fragment_shader_src =
        read_to_string(&fragment).expect(format!("Failed to find {}", &fragment).as_str());

    glium::Program::from_source(
        game.get_display(),
        &vertex_shader_src[..],
        &fragment_shader_src[..],
        None,
    ).expect("Failed to compile shader")
}
