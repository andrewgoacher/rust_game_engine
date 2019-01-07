pub mod matrix;

pub const FOV: f32 = 3.141592 / 3.0;
pub type Vec3 = [f32; 3];
pub type Vec4 = [f32; 4];

pub trait Vector {
    fn zero() -> Self;
    fn new(val: f32) -> Self;
}

impl Vector for Vec3 {
    fn zero() -> Vec3 {
        [0.0f32;3]
    }

    fn new(val: f32) -> Vec3 {
        [val;3]
    }
}

impl Vector for Vec4 {
    fn zero() -> Vec4 {
        [0.0f32;4]
    }

    fn new(val: f32) -> Vec4 {
        [val;4]
    }
}