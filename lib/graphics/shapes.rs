use glium::{VertexBuffer,Display};
use graphics::VertexPositionNormalTexture;

implement_vertex!(VertexPositionNormalTexture, position, normal, texture);

pub fn create_billboard(display: &Display) -> VertexBuffer<VertexPositionNormalTexture> {
    VertexBuffer::new(
        display,
        &[
            VertexPositionNormalTexture {
                position: [-1.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                texture: [0.0, 1.0, 1.0],
            },
            VertexPositionNormalTexture {
                position: [1.0, 1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                texture: [1.0, 1.0, 1.0],
            },
            VertexPositionNormalTexture {
                position: [-1.0, -1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                texture: [0.0, 0.0, 1.0],
            },
            VertexPositionNormalTexture {
                position: [1.0, -1.0, 0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                texture: [1.0, 0.0, 1.0],
            },
        ],
    ).expect("Failed to create billboard")
}