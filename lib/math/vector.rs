pub type Vec2 = [f32; 2];
pub type Vec3 = [f32; 3];
pub type Vec4 = [f32; 4];

pub trait Vector {
    type Err;
    type Result;

    fn zero() -> Self;
    fn from_val(val: f32) -> Self;
    fn from_str(val: &str) -> Result<Self::Result, Self::Err>;
}

impl Vector for Vec3 {
    type Err = String;
    type Result = [f32;3];

    fn zero() -> Vec3 {
        Vec3::from_val(0f32)
    }

    fn from_val(val: f32) -> Vec3 {
        [val, val, val]
    }

    fn from_str(s: &str) -> Result<Self::Result, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        match parts.len() {
            3 => match parts[0].parse::<f32>() {
                Ok(x) => match parts[1].parse::<f32>() {
                    Err(_) => return Err(format!("Couldn't parse y: {}", &parts[1]).to_owned()),
                    Ok(y) => match parts[2].parse::<f32>() {
                        Err(_) => return Err(format!("Couldn't parse z: {}", &parts[2]).to_owned()),
                        Ok(z) => Ok([x, y, z]),
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
    type Result = [f32;4];

    fn zero() -> Vec4 {
        Vec4::from_val(0f32)
    }

    fn from_val(val: f32) -> Vec4 {
        [val, val, val, val]
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
                                return Ok([x, y, z, 1.0f32]);
                            } else {
                                match parts[3].parse::<f32>() {
                                    Err(_) => Ok([x, y, z, 1.0f32]),
                                    Ok(w) => Ok([x, y, z, w]),
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

impl Vector for Vec2 {
    type Err = String;
    type Result = [f32;2];

    fn zero() -> Vec2 {
        Vec2::from_val(0f32)
    }

    fn from_val(val: f32) -> Vec2 {
        [val, val]
    }

    fn from_str(s: &str) -> Result<Self::Result, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        match parts.len() {
            2 => match parts[0].parse::<f32>() {
                Ok(x) => match parts[1].parse::<f32>() {
                    Err(_) => return Err(format!("Couldn't parse y: {}", &parts[1]).to_owned()),
                    Ok(y) => Ok([x, y]),
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
