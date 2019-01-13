//! A module collecting functions and types for representing the description of a renderable mesh
use graphics::Material;
use graphics::{Vertex, VertexPositionNormal, VertexPositionNormalTexture, VertexPositionTexture};
use math::{Vec3, Vec4, ParseVector};
use parser::{FromFile, ParseError};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A struct that represents a single mesh description
#[derive(Clone,Debug)]
pub struct MeshDescription {
    /// The collection of vertices
    pub vertices: Vec<Vertex>,
    /// The name of the mesh
    pub name: String,
    /// The name of the associated material
    pub material: String,
}

// todo: Handle this method better
// todo: Don't just swallow the oks
impl fmt::Display for MeshDescription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Mesh ({})", self.name).ok();
        writeln!(f, "\tMaterial: {}", self.material).ok();
        writeln!(f, "\tVertices: {}", self.vertices.len())
    }
}

/// A struct that represents a collection of materials and a collection of meshes
#[derive(Clone, Debug)]
pub struct MeshDescriptions {
    /// A collection of materials keyed by name
    pub materials: HashMap<String, Material>,
    /// A collection of meshes
    pub meshes: Vec<MeshDescription>,
}

impl fmt::Display for MeshDescriptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(meshes: {}),(materials: {})",
            self.meshes.len(),
            self.materials.len()
        )
    }
}

impl MeshDescription {
    /// Creates a new mesh description
    /// 
    /// # Arguments
    /// `name` - the name of the mesh
    /// `material` - the name of the material
    /// `vertices` - the collection of vertices in the mesh
    fn new(name: String, material: String, vertices: Vec<Vertex>) -> MeshDescription {
        MeshDescription {
            name: name,
            material: material,
            vertices: vertices,
        }
    }
}

impl MeshDescriptions {
    /// Creates a new collection of meshes
    /// 
    /// # Arguments
    /// `meshes` - a collection of meshes
    /// `materials` - a collection of materials
    fn new(meshes: Vec<MeshDescription>, materials: HashMap<String, Material>) -> MeshDescriptions {
        MeshDescriptions {
            materials: materials,
            meshes: meshes,
        }
    }
}

impl FromFile for MeshDescriptions {
    type ParseResult = Result<MeshDescriptions, ParseError>;

    fn from_file(file: &str) -> Self::ParseResult {
        let mut materials: HashMap<String, Material> = HashMap::new();
        let mut current_material: String = "unknown material".to_owned();
        let mut faces: Vec<Vertex> = Vec::new();
        let mut vertex_normals: Vec<Vec3> = Vec::new();
        let mut vertex_textures: Vec<Vec3> = Vec::new();
        let mut vertices: Vec<Vec4> = Vec::new();
        let mut meshes: Vec<MeshDescription> = Vec::new();
        let mut group_name: String = "".to_owned();

        let directory = Path::new(&file)
            .parent()
            .expect("Failed to get parent directory")
            .to_str()
            .expect("Failed to get parent directory string");

        let file = File::open(&file).expect(format!("{} not found!", &file).as_str());
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let parts = match line {
                Ok(ref line) => line[..].split_whitespace().collect::<Vec<&str>>(),
                Err(e) => return Err(ParseError::GeneralError(String::from(format!("{:?}", e)))),
            };

            if parts.len() == 0 {
                continue;
            }

            let (token, rest) = (parts[0], &parts[1..].connect(" "));

            match token {
                "mtllib" => {
                    let material_path = format!("{}/{}", &directory, &rest);
                    materials = match Material::from_file(&material_path) {
                        Ok(m) => m.iter().fold(materials, |mut acc, m| {
                            acc.insert(m.get_name(), m.clone());
                            acc
                        }),
                        Err(e) => match e {
                            ParseError::GeneralError(e) => return Err(ParseError::GeneralError(e)),
                            ParseError::UnknownToken(e) => return Err(ParseError::UnknownToken(e)),
                        },
                    };
                }
                "v" => {
                    match Vec4::from_str(rest) {
                        Ok(arr) => vertices.push(arr),
                        Err(e) => return Err(ParseError::GeneralError(e)),
                    };
                }
                "vn" => {
                    match Vec3::from_str(rest) {
                        Ok(arr) => vertex_normals.push(arr),
                        Err(e) => return Err(ParseError::GeneralError(e)),
                    };
                }
                "vt" => {
                    match Vec3::from_str(rest) {
                        Ok(arr) => vertex_textures.push(arr),
                        Err(e) => return Err(ParseError::GeneralError(e)),
                    };
                }
                "g" => {
                    if faces.len() == 0 {
                        group_name = rest.clone();
                        continue;
                    }

                    meshes.push(MeshDescription::new(
                        group_name,
                        current_material.clone(),
                        faces.clone(),
                    ));
                    group_name = rest.clone();
                    faces.clear();
                }
                "usemtl" => current_material = rest.clone(),
                "f" => match parse_face(rest, &vertices, &vertex_normals, &vertex_textures) {
                    Ok(f) => faces.extend_from_slice(&f[..]),
                    Err(_) => return Err(ParseError::GeneralError("Error parsing face".to_owned())),
                },
                "vp" => continue,
                "s" => continue,
                "#" => continue,
                x => return Err(ParseError::UnknownToken(String::from(x))),
            }
        }

        meshes.push(MeshDescription::new(
            group_name,
            current_material.clone(),
            faces.clone(),
        ));

        Ok(MeshDescriptions::new(meshes, materials))
    }
}

fn parse_face(
    parts: &str,
    positions: &Vec<Vec4>,
    normals: &Vec<Vec3>,
    textures: &Vec<Vec3>,
) -> Result<Vec<Vertex>, ParseError> {
    let mut vertices: Vec<Vertex> = Vec::new();
    let parts = parts.split_whitespace().collect::<Vec<&str>>();

    for part in parts.iter() {
        let s = String::from(*part);

        if s.contains("//") {
            match parse_vertex_normal(&s, &positions, &normals) {
                Ok(vertex_normal) => vertices.push(Vertex::PositionNormal(vertex_normal)),
                Err(e) => return Err(ParseError::GeneralError(String::from(e))),
            };
        } else {
            match parts.len() {
                2 => match parse_vertex_texture(&part, &positions, &textures) {
                    Ok(vertex_texture) => vertices.push(Vertex::PositionTexture(vertex_texture)),
                    Err(e) => return Err(ParseError::GeneralError(String::from(e))),
                },
                _ => match parse_vertex_texture_normal(&part, &positions, &textures, &normals) {
                    Ok(vertex_texture_normal) => {
                        vertices.push(Vertex::PositionNormalTexture(vertex_texture_normal))
                    }
                    Err(e) => return Err(ParseError::GeneralError(String::from(e))),
                },
            }
        }
    }

    Ok(vertices)
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
    parts: &str,
    vertices: &Vec<Vec4>,
    textures: &Vec<Vec3>,
    normals: &Vec<Vec3>,
) -> Result<VertexPositionNormalTexture, String> {
    let parts = parts.split("/").collect::<Vec<&str>>();
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
    parts: &str,
    vertices: &Vec<Vec4>,
    textures: &Vec<Vec3>,
) -> Result<VertexPositionTexture, String> {
    let parts = parts.split("/").collect::<Vec<&str>>();
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
