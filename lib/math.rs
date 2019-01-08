pub const FOV: f32 = 3.141592 / 3.0;
pub type Vec3 = [f32; 3];
pub type Vec4 = [f32; 4];
pub type Mat4x4 = [f32; 16];

pub trait Vector {
    fn zero() -> Self;
    fn new(val: f32) -> Self;
}

pub trait Matrix {
    fn identity() -> Self;
    fn perspective(dimensions: (u32, u32), fov: f32, z: (f32, f32)) -> Self;
    fn view(position: &Vec3, direction: &Vec3, up: &Vec3) -> Self;
    fn to_array(&self) -> [Vec4;4];
}
