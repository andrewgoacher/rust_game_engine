use glium;
use glium::Program;

use game::Engine;
use std::fs::read_to_string;

pub fn create_shader(vertex: &str, fragment: &str, engine: &Engine) -> Program {
    let vertex_shader_src =
        read_to_string(&vertex).expect(format!("Failed to find {}", &vertex).as_str());
    let fragment_shader_src =
        read_to_string(&fragment).expect(format!("Failed to find {}", &fragment).as_str());

    glium::Program::from_source(
        engine.get_display(),
        &vertex_shader_src[..],
        &fragment_shader_src[..],
        None,
    ).expect("Failed to compile shader")
}
