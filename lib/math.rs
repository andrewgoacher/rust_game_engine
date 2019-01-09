use vector::{Vec3,Vec4};

pub const FOV: f32 = 3.141592 / 3.0;

pub type Mat4x4 = [f32; 16];

pub trait Matrix {
    fn identity() -> Self;
    fn perspective(dimensions: (u32, u32), fov: f32, z: (f32, f32)) -> Self;
    fn view(position: &Vec3, direction: &Vec3, up: &Vec3) -> Self;
    fn to_array(&self) -> [[f32;4]; 4];
}