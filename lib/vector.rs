use std::fmt;
use std::ops::Index;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub trait Vector {
    fn zero() -> Self;
    fn from_val(val: f32) -> Self;
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index is out of range"),
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
            _ => panic!("Index is out of range"),
        }
    }
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

impl FromStr for Vec3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl FromStr for Vec4 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
                ).to_owned())
            }
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
