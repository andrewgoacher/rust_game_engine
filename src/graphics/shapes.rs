use glium::backend::Facade;
use glium::VertexBuffer;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

pub fn create_billboard<F>(facade: &F) -> VertexBuffer<Vertex> 
    where F: Facade{
    VertexBuffer::new(facade,
    &[
        Vertex {
            position: [-1.0, 1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
    ]).expect("Failed to create billboard")
}
