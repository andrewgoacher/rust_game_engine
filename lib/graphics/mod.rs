pub mod shapes;

use glium;
use glium::Program;

use std::ffi::OsStr;
use std::path::Path;
use std::fs::{File,read_to_string};

use game::Engine;
use io::to_cursor;

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


fn get_image_format(path: &str) -> Option<image::ImageFormat> {
    let extension = Path::new(&path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    match extension {
        "png" => Some(image::PNG),
        "jpeg" | "jpg" => Some(image::JPEG),
        _ => None
    }
}

pub fn load_texture<'a>(path: &str) -> glium::texture::RawImage2d<'a, u8> {
    let file = File::open(&path).expect(format!("Could not find file {}", &path).as_str());

    let image_format = get_image_format(&path);

    let image = image::load(to_cursor(file), image_format.expect("unexpected extension"))
        .unwrap()
        .to_rgba();

    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

pub trait TextureConvert {
    fn as_texture_2d(self, engine: &Engine) -> glium::texture::Texture2d;
    fn as_srgb_texture_2d(self, engine: &Engine) -> glium::texture::SrgbTexture2d;
}

impl<'a> TextureConvert for glium::texture::RawImage2d<'a, u8> {
    fn as_texture_2d(self, engine: &Engine) -> glium::texture::Texture2d {
        glium::texture::Texture2d::new(engine.get_display(), self).unwrap()
    }

    fn as_srgb_texture_2d(self, engine: &Engine) -> glium::texture::SrgbTexture2d {
        glium::texture::SrgbTexture2d::new(engine.get_display(), self).unwrap()
    }
}
