use math::{Vec3, Vec4};
use std::fmt;

#[derive(Clone, Copy)]
pub struct VertexPositionNormalTexture {
    pub position: Vec4,
    pub normal: Vec3,
    pub texture: Vec3,
}

#[derive(Clone, Copy)]
pub struct VertexPositionTexture {
    pub position: Vec4,
    pub texture: Vec3,
}

#[derive(Clone, Copy)]
pub struct VertexPositionNormal {
    pub position: Vec4,
    pub normal: Vec3,
}

#[derive(Clone, Copy)]
pub enum Vertex {
    PositionNormalTexture(VertexPositionNormalTexture),
    PositionTexture(VertexPositionTexture),
    PositionNormal(VertexPositionNormal),
}

fn format_vec4(v: &Vec4) -> String {
    format!("({},{},{},{})", v[0], v[1], v[2], v[3])
}

fn format_vec3(v: &Vec3) -> String {
    format!("({},{},{})", v[0], v[1], v[2])
}

impl fmt::Display for VertexPositionNormalTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(pos: {})", format_vec4(&self.position));
        writeln!(f, "(normal: {})", format_vec3(&self.normal));
        writeln!(f, "(texture: {})", format_vec3(&self.texture))
    }
}

impl fmt::Display for VertexPositionTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(pos: {})", format_vec4(&self.position));
        writeln!(f, "(texture: {})", format_vec3(&self.texture))
    }
}

impl fmt::Display for VertexPositionNormal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(pos: {})", format_vec4(&self.position));
        writeln!(f, "(normal: {})", format_vec3(&self.normal))
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
