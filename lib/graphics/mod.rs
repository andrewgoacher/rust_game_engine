//! Represents a collection of types and functions for the rendering pipeline
mod material;
mod mesh;
mod vertex;

//todo: Make into prelude
//todo: Make into specific types
pub use self::material::*;
pub use self::mesh::*;
pub use self::vertex::*;

/// Represents the default field of view
pub const FOV: f32 = 3.141592 / 3.0;

use glium::{
    Program, Display,
    vertex::VertexBuffer
};

use std::{
    ffi::OsStr,
    fs::{read_to_string, File},
    path::Path,
};

use io::to_cursor;

// todo: vertex and fragment shader source should be passed in - not file handles 
// todo: Shouldn't return Program, Should return option
/// Creates a shader from a vertex and fragment shader program
/// 
/// # Arguments
/// `vertex` - The vertex shader program file
/// `fragment` - The fragment shader program file
/// `display` - The glium, display & window
/// 
/// returns an OpenGL program
/// 
/// # Panics
/// * When there is no file found for the vertex shader
/// * When there is no file found for the fragment shader
/// * When the program doesn't compile
/// 
/// # Example
/// ```rust,no_run
/// let shader = create_shader(&vertex_shader_src, &fragment_shader_src, &display);
/// ```
pub fn create_shader(vertex: &str, fragment: &str, display: &glium::Display) -> Program {
    let vertex_shader_src =
        read_to_string(&vertex).expect(format!("Failed to find {}", &vertex).as_str());
    let fragment_shader_src =
        read_to_string(&fragment).expect(format!("Failed to find {}", &fragment).as_str());

    glium::Program::from_source(
        display,
        &vertex_shader_src[..],
        &fragment_shader_src[..],
        None,
    )
    .expect("Failed to compile shader")
}

fn get_image_format(path: &str) -> Option<image::ImageFormat> {
    let extension = Path::new(&path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    match extension {
        "png" => Some(image::PNG),
        "jpeg" | "jpg" => Some(image::JPEG),
        _ => None,
    }
}

// todo: Shouldn't load from file, should pass in file
// todo: Make sure unexpected formats are handled better
/// Loads an OpenGL texture from a file as raw image data
/// # Arguments
/// `path` - the path to the texture file
/// 
/// # Panics
/// * When the file could not be found
/// * When the file is in an unsupported format
/// 
/// # Example
/// ```rust,no_run
/// let texture = load_texture("path/to.file");
/// ```
pub fn load_texture<'a>(path: &str) -> glium::texture::RawImage2d<'a, u8> {
    let file = File::open(&path).expect(format!("Could not find file {}", &path).as_str());

    let image_format = get_image_format(&path);

    let image = image::load(to_cursor(file), image_format.expect("unexpected extension"))
        .unwrap()
        .to_rgba();

    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

/// A trait that handles converting from a source type into an OpenGL Texture type
pub trait TextureConvert {
    /// Converts to an OpenGL Texture2d
    /// 
    /// # Arguments
    /// `self`
    /// `display` - The glium display
    /// 
    /// # Example
    /// ```rust,no_run
    /// let texture = load_texture("path/to/texture.file");
    /// let tex_2d = texture.as_texture_2d(&display);
    /// ```
    fn as_texture_2d(self, display: &glium::Display) -> glium::texture::Texture2d;
    /// Converts to an OpenGL Texture2d in SRGB format
    /// 
    /// # Arguments
    /// `self`
    /// `display` - The glium display
    /// 
    /// # Example
    /// ```rust,no_run
    /// let texture = load_texture("path/to_texture.file");
    /// let tex_2d = texture.as_srgb_texture_2d(&display);
    /// ```
    fn as_srgb_texture_2d(self, display: &glium::Display) -> glium::texture::SrgbTexture2d;
}

impl<'a> TextureConvert for glium::texture::RawImage2d<'a, u8> {
    fn as_texture_2d(self, display: &glium::Display) -> glium::texture::Texture2d {
        glium::texture::Texture2d::new(display, self).unwrap()
    }

    fn as_srgb_texture_2d(self, display: &glium::Display) -> glium::texture::SrgbTexture2d {
        glium::texture::SrgbTexture2d::new(display, self).unwrap()
    }
}

implement_vertex!(VertexPositionNormalTexture, position, normal, texture);

/// Creates a 2d square to render a billboarded texture on
/// 
/// # Arguments
/// `display` - The glium display
/// 
/// # Panics
/// when the buffer is not created
/// 
/// # Examples
/// 
/// ```rust,no_run
/// let billboard =  create_billboard(&display);
/// ```
pub fn create_billboard(display: &Display) -> VertexBuffer<VertexPositionNormalTexture> {
    use ::math::{Vec3,Vec4};

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
