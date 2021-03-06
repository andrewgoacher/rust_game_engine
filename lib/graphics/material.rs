//! Module contains a collection of types and functions representing a shader material
//! primary usage of this comes from an obj file parser
use parser::{FromFile, ParseError};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::default::Default;

/// A type that represents a Material
#[derive(Clone, Debug,Default)]
pub struct Material {
    /// Name of the material
    pub name: String,
    /// Specular intensity
    pub ns: f32,
    /// Optical intensity
    pub ni: f32,
    /// Dissolve factor (opacity)
    pub d: f32,
    /// Transparency factor (should be inverse of D)
    pub tr: f32,
    /// Transmission filter
    pub tf: MaterialColor,
    /// Illumination model
    pub illum: IlluminationModel,
    /// Ambient reflection
    pub ka: MaterialColor,
    /// Diffuse reflection
    pub kd: MaterialColor,
    /// Specular reflection
    pub ks: MaterialColor,
    /// Emissive reflection
    pub ke: MaterialColor,
    /// Ambient texture map
    pub map_ka: Option<String>,
    /// Diffise texture map
    pub map_kd: Option<String>,
    /// Reflection map
    pub map_refl: Option<String>,
    /// Emissive map
    pub map_ke: Option<String>,
    /// Bump map
    pub map_bump: Option<String>,
    /// Opacity map
    pub map_d: Option<String>,
}

impl Material {
    /// Gets a copy of the name of the material without moving the borrow.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl FromFile for Material {
    type ParseResult = Result<Vec<Material>, ParseError>;

    fn from_file(file: &str) -> Self::ParseResult {
        let directory = Path::new(&file)
            .parent()
            .expect("Failed to get parent directory")
            .to_str()
            .expect("Failed to get parent directory string");

        let file = File::open(&file).expect(format!("{} not found!", &file).as_str());
        let reader = BufReader::new(&file);
        let mut materials: Vec<Material> = Vec::new();
        let mut material_name: Option<String> = None;
        let mut specular_exponent = 0f32;
        let mut optical_density = 0f32;
        let mut d_factor = 1.0f32;
        let mut transparency = 0f32;
        let mut illum = IlluminationModel::ColorOnAmbientOn;
        let mut ka = MaterialColor::None;
        let mut kd = MaterialColor::None;
        let mut ks = MaterialColor::None;
        let mut ke = MaterialColor::None;
        let mut tf = MaterialColor::None;
        let mut map_ka: Option<String> = None;
        let mut map_kd: Option<String> = None;
        let mut map_refl: Option<String> = None;
        let mut map_ke: Option<String> = None;
        let mut map_bump: Option<String> = None;
        let mut map_d: Option<String> = None;

        for line in reader.lines() {
            let parts = match line {
                Ok(ref line) => line[..].split_whitespace().collect::<Vec<&str>>(),
                Err(e) => return Err(ParseError::GeneralError(String::from(format!("{:?}", e)))),
            };

            if parts.len() == 0 {
                continue;
            }

            let (token, rest) = (parts[0], &parts[1..]);

            match token {
            "newmtl" => {
                match material_name {
                    None => (),
                    Some(s) => {
                        materials.push(Material {
                            name: s,
                            ns: specular_exponent,
                            ni: optical_density,
                            d: d_factor,
                            tr: transparency,
                            tf: tf.clone(),
                            illum: illum.clone(),
                            ka: ka.clone(),
                            kd: kd.clone(),
                            ks: ks.clone(),
                            ke: ke.clone(),
                            map_ka: match map_ka.clone() {
                                None => None,
                                Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                            },
                            map_kd: match map_kd.clone() {
                                None => None,
                                Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                            },
                            map_refl: match map_refl.clone() {
                                None => None,
                                Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                            },
                             map_ke: match map_ke.clone() {
                                None => None,
                                Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                            },
                               map_bump: match map_bump.clone() {
                                None => None,
                                Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                            },
                             map_d: match map_d.clone() {
                                None => None,
                                Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                            },
                        });
                        specular_exponent = 0f32;
                        optical_density = 0f32;
                        d_factor = 1.0f32;
                        transparency = 0f32;
                        illum = IlluminationModel::ColorOnAmbientOn;
                        ka = MaterialColor::None;
                        kd = MaterialColor::None;
                        ks = MaterialColor::None;
                        ke = MaterialColor::None;
                        tf = MaterialColor::None;
                        map_ka = None;
                        map_kd = None;
                        map_refl = None;
                        map_ke = None;
                        map_bump = None;
                        map_d = None;
                    }
                }
                material_name = Some(String::from(rest[0]));
            }
            "Ns" => {
                specular_exponent = match rest[0].parse::<f32>() {
                    Ok(exponent) => exponent,
                    Err(_) => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse specular exponent".to_owned(),
                        ))
                    }
                }
            }
            "Ni" => {
                optical_density = match rest[0].parse::<f32>() {
                    Ok(density) => density,
                    Err(_) => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse optical density".to_owned(),
                        ))
                    }
                }
            }
            "d" => {
                d_factor = match rest[0].parse::<f32>() {
                    Ok(factor) => factor,
                    Err(_) => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse d factor".to_owned(),
                        ))
                    }
                }
            }
            "Tr" => {
                transparency = match rest[0].parse::<f32>() {
                    Ok(t) => t,
                    Err(_) => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse transparency".to_owned(),
                        ))
                    }
                }
            }
            "illum" => illum = match rest[0].parse::<u32>() {
                Ok(i) => match i {
                    0 => IlluminationModel::ColorOnAmbientOff,
                    1 => IlluminationModel::ColorOnAmbientOn,
                    2 => IlluminationModel::HighlightOn,
                    3 => IlluminationModel::ReflectionAndRaytraceOn,
                    4 => IlluminationModel::TransparencyGlassOnReflectionRaytraceOn,
                    5 => IlluminationModel::ReflectionFresnelOnRaytraceOn,
                    6 => IlluminationModel::TransparencyRefractionOnReflectionFresnelOffRaytraceOn,
                    7 => IlluminationModel::TransparencyRefractionOnReflectionFresnelOnRaytraceOn,
                    8 => IlluminationModel::ReflectionOnRaytraceOff,
                    9 => IlluminationModel::TransparencyGlassOnReflectionRaytraceOff,
                    10 => IlluminationModel::CastsShadowsOntoInvisibleSurfacess,
                    _ => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse illumination model".to_owned(),
                        ))
                    }
                },
                Err(_) => {
                    return Err(ParseError::GeneralError(
                        "Failed to parse illumination model".to_owned(),
                    ))
                }
            },
            "Ka" => {
                ka = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse Ka".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "Kd" => {
                kd = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse Kd".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "Ks" => {
                ks = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse Ks".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "Ke" => {
                ke = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse Ke".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "Tf" => {
                tf = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(ParseError::GeneralError(
                            "Failed to parse Tf".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "map_Ka" => map_ka = Some(String::from(rest[0])),
            "map_Kd" => map_kd = Some(String::from(rest[0])),
            "map_refl" => map_refl = Some(String::from(rest[0])),
            "map_Ke" => map_ke = Some(String::from(rest[0])),
            "map_d" => map_d = Some(String::from(rest[0])),
            "map_bump" | "bump" => map_ke = Some(String::from(rest[0])),
            "#" => continue,
            x => {
                return Err(ParseError::UnknownToken(
                    format!("Material parse: unknown token {}", x).to_owned(),
                ))
            }
        }
        }

        match material_name {
            None => (),
            Some(s) => {
                materials.push(Material {
                    name: s,
                    ns: specular_exponent,
                    ni: optical_density,
                    d: d_factor,
                    tr: transparency,
                    tf: tf.clone(),
                    illum: illum.clone(),
                    ka: ka.clone(),
                    kd: kd.clone(),
                    ks: ks.clone(),
                    ke: ke.clone(),
                    map_ka: match map_ka.clone() {
                        None => None,
                        Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                    },
                    map_kd: match map_kd.clone() {
                        None => None,
                        Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                    },
                    map_refl: match map_refl.clone() {
                        None => None,
                        Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                    },
                    map_ke: match map_ke.clone() {
                        None => None,
                        Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                    },
                    map_bump: match map_bump.clone() {
                        None => None,
                        Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                    },
                    map_d: match map_d.clone() {
                        None => None,
                        Some(m) => Some(format!("{}/{}", &directory, m).to_owned()),
                    },
                });
            }
        }

        Ok(materials)
    }
}

fn parse_color(parts: &[&str]) -> MaterialColor {
    match parts[0] {
        "spectral" => {
            let file = String::from(parts[1]);
            let factor = match parts.len() {
                2 => Some(parts[2].parse::<f32>().expect("factor should exist")),
                _ => None,
            };
            MaterialColor::Spectral(file, factor)
        }
        "xyz" => {
            let x = parts[1]
                .parse::<f32>()
                .expect("Failed to parse ciexyz x value");
            let y = parts[2]
                .parse::<f32>()
                .expect("Failed to parse ciexyz y value");
            let z = parts[3]
                .parse::<f32>()
                .expect("Failed to parse ciexyz z value");

            MaterialColor::CIEXYZ(x, y, z)
        }
        r => {
            let r = r
                .parse::<f32>()
                .expect(format!("Failed to parse r value: {}", &r).as_str());
            let g = parts[1]
                .parse::<f32>()
                .expect(format!("Failed to parse r value: {}", &parts[1]).as_str());
            let b = parts[2]
                .parse::<f32>()
                .expect(format!("Failed to parse r value: {}", &parts[2]).as_str());

            MaterialColor::RGB(r, g, b)
        }
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Material ({})", self.name);
        writeln!(f, "\tNs: {}", self.ns);
        writeln!(f, "\tNi: {}", self.ni);
        writeln!(f, "\td: {}", self.d);
        writeln!(f, "\tTr: {}", self.tr);
        writeln!(f, "\tTf: {}", self.tf);
        writeln!(f, "\tillum: {}", self.illum);
        writeln!(f, "\tKa: {}", self.ka);
        writeln!(f, "\tKd: {}", self.kd);
        writeln!(f, "\tKs: {}", self.ks);
        writeln!(f, "\tKe: {}", self.ke);

        match self.map_ka.clone() {
            None => Ok(()),
            Some(m) => writeln!(f, "\tmap_ka: {}", m),
        };

        match self.map_kd.clone() {
            None => Ok(()),
            Some(m) => writeln!(f, "\tmap_kd: {}", m),
        };

        match self.map_ke.clone() {
            None => Ok(()),
            Some(m) => writeln!(f, "\tmap_ke: {}", m),
        };

        match self.map_bump.clone() {
            None => Ok(()),
            Some(m) => writeln!(f, "\tmap_bump: {}", m),
        };

        match self.map_d.clone() {
            None => Ok(()),
            Some(m) => writeln!(f, "\tmap_d: {}", m),
        };

        match self.map_refl.clone() {
            None => Ok(()),
            Some(m) => writeln!(f, "\tmap_refl: {}", m),
        }
    }
}

// todo: Material and obj loading might need to be made into a more generic feature.
/// An enum representing a materials illumination model
#[derive(Clone, Debug)]
pub enum IlluminationModel {
    /// 0 - Color on and Ambient off
    ColorOnAmbientOff,
    /// 1 - Color on and Ambient on
    ColorOnAmbientOn,
    /// 2 - Highlight on
    HighlightOn,
    /// 3 - Reflection on and raytrace on
    ReflectionAndRaytraceOn,
    /// 4 - Transparency: Glass on, Reflection: Raytrace on
    TransparencyGlassOnReflectionRaytraceOn,
    /// 5 - Reflection: Fresnel on and Raytrace on
    ReflectionFresnelOnRaytraceOn,
    /// 6 - Transparency: Refraction on, Reflection: Fresnel off and Raytrace on
    TransparencyRefractionOnReflectionFresnelOffRaytraceOn,
    /// 7 - Transparency: Refraction on, Reflection: Fresnel on and Raytrace on
    TransparencyRefractionOnReflectionFresnelOnRaytraceOn,
    /// 8 - Reflection on and Raytrace off
    ReflectionOnRaytraceOff,
    /// 9 - Transparency: Flass on, Reflection: Raytrace off
    TransparencyGlassOnReflectionRaytraceOff,
    /// 10 - Casts shadows onto invisible surfaces
    CastsShadowsOntoInvisibleSurfacess,
}

impl Default for IlluminationModel {
    fn default() -> IlluminationModel {
        IlluminationModel::ColorOnAmbientOff
    }
}

impl fmt::Display for IlluminationModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IlluminationModel::ColorOnAmbientOff => write!(f, "0. Color on and Ambient off"),
            IlluminationModel::ColorOnAmbientOn => write!(f, "1. Color on and Ambient on"),
            IlluminationModel::HighlightOn => write!(f, "2. Highlight on"),
            IlluminationModel::ReflectionAndRaytraceOn => {
                write!(f, "3. Reflection on and Ray trace on")
            }
            IlluminationModel::TransparencyGlassOnReflectionRaytraceOn => {
                write!(f, "4. Transparency: Glass on, Reflection: Ray trace on")
            }
            IlluminationModel::ReflectionFresnelOnRaytraceOn => {
                write!(f, "5. Reflection: Fresnel on and Ray trace on")
            }
            IlluminationModel::TransparencyRefractionOnReflectionFresnelOffRaytraceOn => write!(
                f,
                "6. Transparency: Refraction on, Reflection: Fresnel off and Ray trace on"
            ),
            IlluminationModel::TransparencyRefractionOnReflectionFresnelOnRaytraceOn => write!(
                f,
                "7. Transparency: Refraction on, Reflection: Fresnel on and Ray trace on"
            ),
            IlluminationModel::ReflectionOnRaytraceOff => {
                write!(f, "8. Reflection on and Ray trace off")
            }
            IlluminationModel::TransparencyGlassOnReflectionRaytraceOff => {
                write!(f, "9. Transparency: Glass on, Reflection: Ray trace off")
            }
            IlluminationModel::CastsShadowsOntoInvisibleSurfacess => {
                write!(f, "10. Casts shadows onto invisible surfaces")
            }
        }
    }
}

#[derive(Clone, Debug)]
/// An enum representing various color formats
pub enum MaterialColor {
    /// No color
    None,
    /// RGB standard format
    /// # Arguments
    /// `r` - The red value
    /// `g` - The green value
    /// `b` - The blue value
    RGB(f32, f32, f32),
    /// CIE format
    /// [See Wiki](https://en.wikipedia.org/wiki/CIE_1931_color_space)
  
    CIEXYZ(f32, f32, f32),
    /// Spectral format
    /// # Arguments
    /// `file` - The file to load
    /// `factor` - an optional factor
    Spectral(String, Option<f32>),
}

impl Default for MaterialColor {
    fn default() -> MaterialColor {
        MaterialColor::None
    }
}

impl fmt::Display for MaterialColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MaterialColor::None => write!(f, "None"),
            MaterialColor::RGB(r, g, b) => write!(f, "rgb({},{},{})", r, g, b),
            MaterialColor::CIEXYZ(x, y, z) => write!(f, "ciexyz({},{},{})", x, y, z),
            MaterialColor::Spectral(file, factor) => match factor {
                None => write!(f, "spectral (no factor): {}", file),
                Some(factor) => write!(f, "spectral: {} ({})", file, factor),
            },
        }
    }
}
