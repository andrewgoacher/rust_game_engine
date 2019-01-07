use math::{Vec3, Vec4};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Meshes {
    materials: HashMap<String, Material>,
    meshes: Vec<Mesh>,
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    name: String,
    material: String,
}

pub trait Printable {
    fn print(&self) -> ();
}

impl Mesh {
    fn new(name: String, material: String, vertices: Vec<Vertex>) -> Mesh {
        Mesh {
            name: name,
            material: material,
            vertices: vertices,
        }
    }
}

impl Printable for Mesh {
    fn print(&self) -> () {
        println!("\t\tMesh: {}", self.name);
        println!("\t\t\tmaterial: {}", self.material);
        println!("\t\t\tNum vertices: {}", self.vertices.len());
    }
}

impl Printable for Meshes {
    fn print(&self) -> () {
        println!("Total materials: {}", self.materials.len());

        for (k, m) in self.materials.iter() {
            m.print();
        }

        println!("Total meshes: {}", self.meshes.len());
        for mesh in self.meshes.iter() {
            mesh.print();
        }
    }
}

impl Printable for Material {
    fn print(&self) -> () {
        println!("\t\tMaterial: {}", self.name);
        println!("\t\t\tNi: {}", self.ni);
        println!("\t\t\tNs: {}", self.ns);
        println!("\t\t\td: {}", self.d);
        println!("\t\t\tTr: {}", self.tr);
        print!("\t\t\ttf: ");
        self.tf.print();
        println!("\t\t\tillum: {:?}", self.illum);
        print!("\t\t\tka: ");
        self.ka.print();
        print!("\t\t\tkd: ");
        self.kd.print();
        print!("\t\t\tks: ");
        self.ks.print();
        print!("\t\t\tke: ");
        self.ke.print();

        match self.map_ka.clone() {
            None => (),
            Some(m) => println!("\t\t\tmap_ka: {}", m)
        };

        match self.map_kd.clone() {
            None => (),
            Some(m) => println!("\t\t\tmap_kd: {}", m)
        };

        match self.map_refl.clone() {
            None => (),
            Some(m) => println!("\t\t\tmap_refl: {}", m)
        };
    }
}

#[derive(Debug, Clone)]
pub struct VertexPositionNormalTexture {
    position: Vec4,
    normal: Vec3,
    texture: Vec3,
}

#[derive(Debug, Clone)]
pub struct VertexPositionTexture {
    position: Vec4,
    texture: Vec3,
}

#[derive(Debug, Clone)]
pub struct VertexPositionNormal {
    position: Vec4,
    normal: Vec3,
}

#[derive(Debug, Clone)]
pub enum Vertex {
    PositionNormalTexture(VertexPositionNormalTexture),
    PositionTexture(VertexPositionTexture),
    PositionNormal(VertexPositionNormal),
}

pub enum MeshLoadError {
    ParseError(String),
    UnknownTokenError(String),
}

pub type MeshLoadResult = Result<Meshes, MeshLoadError>;
type ParseFloatResult<T> = Result<T, MeshLoadError>;
type ParseFaceResult = Result<Vec<Vertex>, MeshLoadError>;

fn parse_float4(parts: &[&str], default_w: f32) -> ParseFloatResult<Vec4> {
    let mut result = [default_w; 4];
    for (i, p) in parts.iter().enumerate() {
        match p.parse::<f32>() {
            Ok(f) => result[i] = f,
            Err(_) => {
                return Err(MeshLoadError::ParseError(String::from(
                    "(float4): Couldn't parse floats",
                )))
            }
        };
    }
    Ok(result)
}

fn parse_float3(parts: &[&str], default: f32) -> ParseFloatResult<Vec3> {
    let mut result = [default; 3];
    for (i, p) in parts.iter().enumerate() {
        match p.parse::<f32>() {
            Ok(f) => result[i] = f,
            Err(_) => {
                return Err(MeshLoadError::ParseError(String::from(
                    "(float3): Couldn't parse floats",
                )))
            }
        };
    }
    Ok(result)
}

fn parse_vertex_normal(
    s: &String,
    vertices: &Vec<Vec4>,
    normals: &Vec<Vec3>,
) -> Result<VertexPositionNormal, String> {
    let parts = s.split("//").collect::<Vec<&str>>();

    match parts[0].parse::<usize>() {
        Ok(v_index) => match parts[1].parse::<usize>() {
            Ok(n_index) => Ok(VertexPositionNormal {
                position: vertices[v_index - 1],
                normal: normals[n_index - 1],
            }),
            Err(_) => Err("Error parsing vertex normal".to_owned()),
        },
        Err(_) => Err("Error parsing vertex normal".to_owned()),
    }
}

fn parse_vertex_texture_normal(
    parts: &[&str],
    vertices: &Vec<Vec4>,
    textures: &Vec<Vec3>,
    normals: &Vec<Vec3>,
) -> Result<VertexPositionNormalTexture, String> {
    match parts[0].parse::<usize>() {
        Ok(v_index) => match parts[1].parse::<usize>() {
            Ok(t_index) => match parts[2].parse::<usize>() {
                Ok(n_index) => Ok(VertexPositionNormalTexture {
                    position: vertices[v_index - 1],
                    normal: normals[n_index - 1],
                    texture: textures[t_index - 1],
                }),
                Err(_) => Err("Error parsing vertex texture normal".to_owned()),
            },
            Err(_) => Err("Error parsing vertex texture normal".to_owned()),
        },
        Err(_) => Err("Error parsing vertex texture normal".to_owned()),
    }
}

fn parse_vertex_texture(
    parts: &[&str],
    vertices: &Vec<Vec4>,
    textures: &Vec<Vec3>,
) -> Result<VertexPositionTexture, String> {
    match parts[0].parse::<usize>() {
        Ok(v_index) => match parts[1].parse::<usize>() {
            Ok(t_index) => Ok(VertexPositionTexture {
                position: vertices[v_index - 1],
                texture: textures[t_index - 1],
            }),
            Err(_) => Err("Error parsing vertex texture".to_owned()),
        },
        Err(_) => Err("Error parsing vertex texture".to_owned()),
    }
}

fn parse_face(
    parts: &[&str],
    positions: &Vec<Vec4>,
    normals: &Vec<Vec3>,
    textures: &Vec<Vec3>,
) -> ParseFaceResult {
    let mut vertices: Vec<Vertex> = Vec::new();

    for part in parts.iter() {
        let s = String::from(*part);

        if s.contains("//") {
            match parse_vertex_normal(&s, &positions, &normals) {
                Ok(vertex_normal) => vertices.push(Vertex::PositionNormal(vertex_normal)),
                Err(e) => return Err(MeshLoadError::ParseError(String::from(e))),
            };
        } else {
            let parts = s.split("/").collect::<Vec<&str>>();
            match parts.len() {
                2 => match parse_vertex_texture(&parts, &positions, &textures) {
                    Ok(vertex_texture) => vertices.push(Vertex::PositionTexture(vertex_texture)),
                    Err(e) => return Err(MeshLoadError::ParseError(String::from(e))),
                },
                _ => match parse_vertex_texture_normal(&parts, &positions, &textures, &normals) {
                    Ok(vertex_texture_normal) => {
                        vertices.push(Vertex::PositionNormalTexture(vertex_texture_normal))
                    }
                    Err(e) => return Err(MeshLoadError::ParseError(String::from(e))),
                },
            }
        }
    }

    Ok(vertices)
}
#[derive(Clone, Debug)]
pub struct Material {
    name: String,
    ns: f32,
    ni: f32,
    d: f32,
    tr: f32,
    tf: MaterialColor,
    illum: IlluminationModel,
    ka: MaterialColor,
    kd: MaterialColor,
    ks: MaterialColor,
    ke: MaterialColor,
    map_ka: Option<String>,
    map_kd: Option<String>,
    map_refl: Option<String>,
}

impl Material {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

/*
0. Color on and Ambient off
1. Color on and Ambient on
2. Highlight on
3. Reflection on and Ray trace on
4. Transparency: Glass on, Reflection: Ray trace on
5. Reflection: Fresnel on and Ray trace on
6. Transparency: Refraction on, Reflection: Fresnel off and Ray trace on
7. Transparency: Refraction on, Reflection: Fresnel on and Ray trace on
8. Reflection on and Ray trace off
9. Transparency: Glass on, Reflection: Ray trace off
10. Casts shadows onto invisible surfaces
*/

#[derive(Clone, Debug)]
pub enum IlluminationModel {
    ColorOnAmbientOff,
    ColorOnAmbientOn,
    HighlightOn,
    ReflectionAndRaytraceOn,
    TransparencyGlassOnReflectionRaytraceOn,
    ReflectionFresnelOnRaytraceOn,
    TransparencyRefractionOnReflectionFresnelOffRaytraceOn,
    TransparencyRefractionOnReflectionFresnelOnRaytraceOn,
    ReflectionOnRaytraceOff,
    TransparencyGlassOnReflectionRaytraceOff,
    CastsShadowsOntoInvisibleSurfacess,
}

#[derive(Clone, Debug)]
pub enum MaterialColor {
    None,
    RGB(f32, f32, f32),
    CIEXYZ(f32, f32, f32),
    Spectral(String, Option<f32>),
}

impl Printable for MaterialColor {
    fn print(&self) -> () {
        match self {
            MaterialColor::None => (),
            MaterialColor::RGB(r,g,b) => println!("(r,g,b): ({},{},{})",r,g,b),
            MaterialColor::CIEXYZ(x,y,z) =>  println!("(x,y,z): ({},{},{})",x,y,z),
            MaterialColor::Spectral(file, factor) => match factor {
                None => println!("spectral: {}", file),
                Some(f) => println!("(spectral,factor): ({}, {})", file, f)
            }
        }
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

fn parse_materials(file: &str, directory: &str) -> MaterialsResult {
    let file = File::open(&file).expect(format!("{} not found!", &file).as_str());
    let mut reader = BufReader::new(&file);
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

    for line in reader.lines() {
        let parts = match line {
            Ok(ref line) => line[..].split_whitespace().collect::<Vec<&str>>(),
            Err(e) => {
                return Err(MaterialLoadError::ParseError(String::from(format!(
                    "{:?}",
                    e
                ))))
            }
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
                    }
                }
                material_name = Some(String::from(rest[0]));
            }
            "Ns" => {
                specular_exponent = match rest[0].parse::<f32>() {
                    Ok(exponent) => exponent,
                    Err(_) => {
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse specular exponent".to_owned(),
                        ))
                    }
                }
            }
            "Ni" => {
                optical_density = match rest[0].parse::<f32>() {
                    Ok(density) => density,
                    Err(_) => {
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse optical density".to_owned(),
                        ))
                    }
                }
            }
            "d" => {
                d_factor = match rest[0].parse::<f32>() {
                    Ok(factor) => factor,
                    Err(_) => {
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse d factor".to_owned(),
                        ))
                    }
                }
            }
            "Tr" => {
                transparency = match rest[0].parse::<f32>() {
                    Ok(t) => t,
                    Err(_) => {
                        return Err(MaterialLoadError::ParseError(
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
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse illumination model".to_owned(),
                        ))
                    }
                },
                Err(_) => {
                    return Err(MaterialLoadError::ParseError(
                        "Failed to parse illumination model".to_owned(),
                    ))
                }
            },
            "Ka" => {
                ka = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse Ka".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "Kd" => {
                kd = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse Kd".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "Ks" => {
                ks = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse Ks".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "Ke" => {
                ke = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse Ke".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "Tf" => {
                tf = match parse_color(&rest) {
                    MaterialColor::None => {
                        return Err(MaterialLoadError::ParseError(
                            "Failed to parse Tf".to_owned(),
                        ))
                    }
                    x => x,
                }
            }
            "map_Ka" => map_ka = Some(String::from(rest[0])),
            "map_Kd" => map_kd = Some(String::from(rest[0])),
            "map_refl" => map_refl = Some(String::from(rest[0])),
            "#" => continue,
            x => {
                return Err(MaterialLoadError::UnknownTokenError(
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
            });
        }
    }

    Ok(materials)
}

pub type MaterialsResult = Result<Vec<Material>, MaterialLoadError>;
pub type MaterialResult = Result<Material, MaterialLoadError>;
pub enum MaterialLoadError {
    ParseError(String),
    UnknownTokenError(String),
}

impl Meshes {
    pub fn load(file: &str) -> MeshLoadResult {
        use std::time::SystemTime;
        let now = SystemTime::now();

        let mut materials: HashMap<String, Material> = HashMap::new();
        let mut current_material: String = "unknown material".to_owned();
        let mut faces: Vec<Vertex> = Vec::new();
        let mut vertex_normals: Vec<Vec3> = Vec::new();
        let mut vertex_textures: Vec<Vec3> = Vec::new();
        let mut vertices: Vec<Vec4> = Vec::new();
        let mut meshes: Vec<Mesh> = Vec::new();
        let mut group_name: String = "".to_owned();

        let directory = Path::new(&file)
            .parent()
            .expect("Failed to get parent directory")
            .to_str()
            .expect("Failed to get parent directory string");

        let file = File::open(&file).expect(format!("{} not found!", &file).as_str());
        let mut reader = BufReader::new(&file);

        for line in reader.lines() {
            let parts = match line {
                Ok(ref line) => line[..].split_whitespace().collect::<Vec<&str>>(),
                Err(e) => return Err(MeshLoadError::ParseError(String::from(format!("{:?}", e)))),
            };

            if parts.len() == 0 {
                continue;
            }

            let (token, rest) = (parts[0], &parts[1..]);

            match token {
                "mtllib" => {
                    let material_path = format!("{}/{}", &directory, &rest[0]);
                    materials = match parse_materials(&material_path, &directory) {
                        Ok(m) => m.iter().fold(materials, |mut acc, m| {
                            acc.insert(m.get_name(), m.clone());
                            acc
                        }),
                        Err(e) => match e {
                            MaterialLoadError::ParseError(e) => {
                                return Err(MeshLoadError::ParseError(e))
                            }
                            MaterialLoadError::UnknownTokenError(e) => {
                                return Err(MeshLoadError::UnknownTokenError(e))
                            }
                        },
                    };
                }
                "v" => {
                    match parse_float4(rest, 1.0f32) {
                        Ok(arr) => vertices.push(arr),
                        Err(e) => return Err(e),
                    };
                }
                "vn" => {
                    match parse_float3(rest, 1.0f32) {
                        Ok(arr) => vertex_normals.push(arr),
                        Err(e) => return Err(e),
                    };
                }
                "vt" => {
                    match parse_float3(rest, 0.0f32) {
                        Ok(arr) => vertex_textures.push(arr),
                        Err(e) => return Err(e),
                    };
                }
                "g" => {
                    if faces.len() == 0 {
                        group_name = String::from(rest[0]);
                        continue;
                    }

                    meshes.push(Mesh::new(
                        group_name,
                        current_material.clone(),
                        faces.clone(),
                    ));
                    group_name = String::from(rest[0]);
                    faces.clear();
                }
                "usemtl" => current_material = String::from(rest[0]),
                "f" => match parse_face(rest, &vertices, &vertex_normals, &vertex_textures) {
                    Ok(f) => faces.extend_from_slice(&f[..]),
                    Err(_) => {
                        return Err(MeshLoadError::ParseError("Error parsing face".to_owned()))
                    }
                },
                "vp" => continue,
                "s" => continue,
                "#" => continue,
                x => return Err(MeshLoadError::UnknownTokenError(String::from(x))),
            }
        }

        meshes.push(Mesh::new(
            group_name,
            current_material.clone(),
            faces.clone(),
        ));

        match now.elapsed() {
            Ok(elapsed) => {
                println!("Time taken to parse: {} seconds", elapsed.as_secs());
            }
            Err(e) => {
                println!("Failed to get timer: {:?}", e);
            }
        }

        Ok(Meshes {
            meshes: meshes,
            materials: materials,
        })
    }
}
