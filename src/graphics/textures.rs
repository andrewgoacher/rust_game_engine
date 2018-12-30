use glium;
use image::ImageFormat;

use io::to_cursor;
use std::fs::File;

pub fn load_texture_srgb<F>(
    facade: &F,
    file_name: &str,
    fmt: ImageFormat,
) -> glium::texture::SrgbTexture2d
where
    F: glium::backend::Facade,
{
    let file =
        File::open(&file_name).expect(format!("Could not find file {}", &file_name).as_str());
    let image = image::load(to_cursor(file), fmt).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::SrgbTexture2d::new(facade, image).unwrap()
}

pub fn load_texture<F>(
    facade: &F,
    file_name: &str,
    fmt: ImageFormat,
) -> glium::texture::Texture2d
where
    F: glium::backend::Facade,
{
    let file =
        File::open(&file_name).expect(format!("Could not find file {}", &file_name).as_str());
    let image = image::load(to_cursor(file), fmt).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(facade, image).unwrap()
}
