use glium::{Display, VertexBuffer};
use graphics::VertexPositionNormalTexture;
use math::{Vec3, Vec4};

implement_vertex!(VertexPositionNormalTexture, position, normal, texture);

pub fn create_billboard(display: &Display) -> VertexBuffer<VertexPositionNormalTexture> {
    VertexBuffer::new(
        display,
        &[
            VertexPositionNormalTexture {
                position: Vec4 {
                    x: -1.0,
                    y: 1.0,
                    z: 0.0,
                    w: 1.0,
                },
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                texture: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            VertexPositionNormalTexture {
                position: Vec4 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                    w: 1.0,
                },
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                texture: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
            },
            VertexPositionNormalTexture {
                position: Vec4 {
                    x: -1.0,
                    y: -1.0,
                    z: 0.0,
                    w: 1.0,
                },
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                texture: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            },
            VertexPositionNormalTexture {
                position: Vec4 {
                    x: 1.0,
                    y: -1.0,
                    z: 0.0,
                    w: 1.0,
                },
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                texture: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
            },
        ],
    )
    .expect("Failed to create billboard")
}
