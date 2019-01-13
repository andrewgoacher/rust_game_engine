//! A module containing Vectors.
//! A Vector is an object that will contain a set of points.
//!
//! An example is a Vec2 which is an object representing 2 points (x and y)
//! or a vec3 and object representing 3 points (x,y and z)
use glium::vertex::{Attribute, AttributeType};
use std::ops::Index;

/// A struct representing a 2 point vector
///
/// # Example
///
/// ```rust
/// let v2 = Vec2 {x: 1.0f32, y: 2.0f32};
/// # assert_eq!(1.0f32, v2.x);
/// # assert_eq!(2.0f32, v2.y);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    /// Represents the first point in the vec2
    pub x: f32,
    /// Represents the second point in the vec2
    pub y: f32,
}

/// A struct representing a 3 point vector
///
/// # Example
///
/// ```rust
/// let v3 = Vec3 {x: 1.0f32, y: 2.0f32, z: 3.0f32};
/// # assert_eq!(1.0f32, v3.x);
/// # assert_eq!(2.0f32, v3.y);
/// # assert_eq!(3.0f32, v3.z);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    /// Represents the first point in the vec3
    pub x: f32,
    /// Represents the second point in the vec3
    pub y: f32,
    /// Represents the third point in the vec3
    pub z: f32,
}

/// A strict representing a 4 point vector
///
/// # Example
///
/// ```rust
/// let v4 = Vec4 { x: 1.0f32, y: 2.0f32, z: 3.0f32, w: 4.0f32}
/// # assert_eq!(1.0f32, v4.x);
/// # assert_eq!(2.0f32, v4.y);
/// # assert_eq!(3.0f32, v4.z);
/// # assert_eq!(4.0f32, v4.w);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
    /// Represents the first point in the vec4
    pub x: f32,
    /// Represents the second point in the vec4
    pub y: f32,
    /// Represents the third point in the vec4
    pub z: f32,
    /// Represents the fourth point in the vec4
    pub w: f32,
}

/// A collection of functions for Vectors
pub trait Vector {
    /// returns an empty vector
    ///
    /// # Example
    ///
    /// ```rust
    /// let v = Vec2::zero();
    /// #assert_eq!(0.0f32, v.x);
    /// #assert_eq!(0.0f32, v.y);
    /// ```
    fn zero() -> Self;

    /// returns a vector with each point set to the provided value
    ///
    /// # Arguments
    /// `val` - the value for each point
    ///
    /// # Example
    /// ```rust
    /// let v = Vec2::from_val(5.0f32);
    /// #assert_eq!(5.0f32, v.x);
    /// #assert_eq!(5.0f32, v.y);
    /// ```
    fn from_val(val: f32) -> Self;
}

/// A collection of functions for parsing vectors
pub trait ParseVector {
    /// Represents the error that can be returned from trying to parse from a string
    type Err;
    /// Represents the result type that is returned from parsing from a string
    type Result;

    /// Attempts to parse a string into the Vector
    /// will reuturn a defined error if it is unsuccessful
    ///
    /// # Arguments
    ///
    /// `val` - the string to parse
    ///
    /// # Example
    ///
    /// ```rust
    /// let s = "1.0 2.0 3.0";
    /// let v = Vec3::from_str(s);
    /// # assert_eq!(1.0f32, v.x);
    /// # assert_eq!(2.0f32, v.y);
    /// # assert_eq!(3.0f32, v.z);
    /// ```
    fn from_str(val: &str) -> Result<Self::Result, Self::Err>;
}

impl Vector for Vec3 {
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
}

impl ParseVector for Vec3 {
    type Err = String;
    type Result = Vec3;

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
}

impl ParseVector for Vec4 {
    type Err = String;
    type Result = Vec4;

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
    fn zero() -> Vec2 {
        Vec2::from_val(0f32)
    }

    fn from_val(val: f32) -> Vec2 {
        Vec2 { x: val, y: val }
    }
}

impl ParseVector for Vec2 {
    type Err = String;
    type Result = Vec2;

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
