use math::{Vec3,Vec4,Vector};

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