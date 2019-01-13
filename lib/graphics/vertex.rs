//! A module for standard Vertex types for use in rendering 3d images
use math::{Vec3, Vec4};
use std::fmt;

/// A vertex that contains a position, a normal and a texture coordinate
#[derive(Clone, Copy,Debug)]
pub struct VertexPositionNormalTexture {
    /// The position of the Vertex
    pub position: Vec4,
    /// The vertex normal
    pub normal: Vec3,
    /// The texture coordinates of the vertex
    pub texture: Vec3,
}

/// A vertex that contains a position and a texture coordinate
#[derive(Clone, Copy,Debug)]
pub struct VertexPositionTexture {
    /// The position of the vertex
    pub position: Vec4,
    /// The texture coordinates of the vertex
    pub texture: Vec3,
}

/// A vertex that contains a position and a normal
#[derive(Clone, Copy,Debug)]
pub struct VertexPositionNormal {
    /// The position of the vertex
    pub position: Vec4,
    /// The vertex normal
    pub normal: Vec3,
}

/// An enum that represents one of a variety of vertex types
#[derive(Clone, Copy,Debug)]
pub enum Vertex {
    /// A vertex that contains a position, normal and texture coordinate
    PositionNormalTexture(VertexPositionNormalTexture),
    /// A vertex that has a position and a texture coordinate
    PositionTexture(VertexPositionTexture),
    /// A vertex that has a position and a normal
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
