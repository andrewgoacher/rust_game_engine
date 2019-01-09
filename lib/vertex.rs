use std::fmt;
use vector::{Vec3, Vec4};

#[derive(Clone)]
pub struct VertexPositionNormalTexture {
    pub position: Vec4,
    pub normal: Vec3,
    pub texture: Vec3,
}

#[derive(Clone)]
pub struct VertexPositionTexture {
    pub position: Vec4,
    pub texture: Vec3,
}

#[derive(Clone)]
pub struct VertexPositionNormal {
    pub position: Vec4,
    pub normal: Vec3,
}

#[derive(Clone)]
pub enum Vertex {
    PositionNormalTexture(VertexPositionNormalTexture),
    PositionTexture(VertexPositionTexture),
    PositionNormal(VertexPositionNormal),
}

impl fmt::Display for VertexPositionNormalTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(pos: {})", self.position);
        writeln!(f, "(normal: {})", self.normal);
        writeln!(f, "(texture: {})", self.texture)
    }
}

impl fmt::Display for VertexPositionTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(pos: {})", self.position);
        writeln!(f, "(texture: {})", self.texture)
    }
}

impl fmt::Display for VertexPositionNormal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(pos: {})", self.position);
        writeln!(f, "(normal: {})", self.normal)
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Vertex::PositionNormalTexture(t) => write!(f, "{}", t),
            Vertex::PositionTexture(t) => write!(f, "{}", t),
            Vertex::PositionNormal(t) => write!(f, "{}", t),
        }
    }
}
