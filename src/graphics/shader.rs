use glium;
use glium::Program;

use std::fs::read_to_string;

pub fn create_shader<'a, F: ?Sized>(vertex: &'a str, fragment: &'a str, facade: &F) -> Program
where
    F: glium::backend::Facade
{
    let vertex_shader_src =
        read_to_string(&vertex).expect(format!("Failed to find {}", &vertex).as_str());
    let fragment_shader_src =
        read_to_string(&fragment).expect(format!("Failed to find {}", &fragment).as_str());

    glium::Program::from_source(
        facade,
        &vertex_shader_src[..],
        &fragment_shader_src[..],
        None,
    ).expect("Failed to compile shader")
}
