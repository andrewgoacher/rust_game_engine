use glium::vertex::{Attribute, AttributeType};
use std::ops::Index;

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait Vector {
    type Err;
    type Result;

    fn zero() -> Self;
    fn from_val(val: f32) -> Self;
    fn from_str(val: &str) -> Result<Self::Result, Self::Err>;
}

impl Vector for Vec3 {
    type Err = String;
    type Result = Vec3;

    fn zero() -> Vec3 {
        Vec3::from_val(0f32)
    }

    fn from_val(val: f32) -> Vec3 {
        Vec3 {
            x: val,
            y: val,
            z: val,
        }
    }

    fn from_str(s: &str) -> Result<Self::Result, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        match parts.len() {
            3 => match parts[0].parse::<f32>() {
                Ok(x) => match parts[1].parse::<f32>() {
                    Err(_) => return Err(format!("Couldn't parse y: {}", &parts[1]).to_owned()),
                    Ok(y) => match parts[2].parse::<f32>() {
                        Err(_) => return Err(format!("Couldn't parse z: {}", &parts[2]).to_owned()),
                        Ok(z) => Ok(Vec3 { x: x, y: y, z: z }),
                    },
                },
                Err(_) => return Err(format!("Couldn't parse x: {}", &parts[0]).to_owned()),
            },
            _ => {
                return Err(
                    format!("string not in correct format should be `x y z` not {}", &s).to_owned(),
                )
            }
        }
    }
}

impl Vector for Vec4 {
    type Err = String;
    type Result = Vec4;

    fn zero() -> Vec4 {
        Vec4::from_val(0f32)
    }

    fn from_val(val: f32) -> Vec4 {
        Vec4 {
            x: val,
            y: val,
            z: val,
            w: val,
        }
    }

    fn from_str(s: &str) -> Result<Self::Result, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        match parts.len() {
            len if len == 3 || len == 4 => match parts[0].parse::<f32>() {
                Err(_) => return Err(format!("Couldn't parse x: {}", &parts[0]).to_owned()),
                Ok(x) => match parts[1].parse::<f32>() {
                    Err(_) => return Err(format!("Couldn't parse y: {}", &parts[1]).to_owned()),
                    Ok(y) => match parts[2].parse::<f32>() {
                        Err(_) => return Err(format!("Couldn't parse z: {}", &parts[2]).to_owned()),
                        Ok(z) => {
                            if len == 3 {
                                return Ok(Vec4 {
                                    x: x,
                                    y: y,
                                    z: z,
                                    w: 1.0f32,
                                });
                            } else {
                                match parts[3].parse::<f32>() {
                                    Err(_) => Ok(Vec4 {
                                        x: x,
                                        y: y,
                                        z: z,
                                        w: 1.0f32,
                                    }),
                                    Ok(w) => Ok(Vec4 {
                                        x: x,
                                        y: y,
                                        z: z,
                                        w: w,
                                    }),
                                }
                            }
                        }
                    },
                },
            },
            _ => {
                return Err(format!(
                    "string not in correct format should be `x y z w` not {}",
                    &s
                )
                .to_owned())
            }
        }
    }
}

impl Vector for Vec2 {
    type Err = String;
    type Result = Vec2;

    fn zero() -> Vec2 {
        Vec2::from_val(0f32)
    }

    fn from_val(val: f32) -> Vec2 {
        Vec2 { x: val, y: val }
    }

    fn from_str(s: &str) -> Result<Self::Result, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        match parts.len() {
            2 => match parts[0].parse::<f32>() {
                Ok(x) => match parts[1].parse::<f32>() {
                    Err(_) => return Err(format!("Couldn't parse y: {}", &parts[1]).to_owned()),
                    Ok(y) => Ok(Vec2 { x: x, y: y }),
                },
                Err(_) => return Err(format!("Couldn't parse x: {}", &parts[0]).to_owned()),
            },
            _ => {
                return Err(
                    format!("string not in correct format should be `x y z` not {}", &s).to_owned(),
                )
            }
        }
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of range"),
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of range"),
        }
    }
}

unsafe impl Attribute for Vec2 {
    fn get_type() -> AttributeType {
        AttributeType::F32F32
    }
}

unsafe impl Attribute for Vec3 {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32
    }
}

unsafe impl Attribute for Vec4 {
    fn get_type() -> AttributeType {
        AttributeType::F32F32F32F32
    }
}
