use math::{Vec3, Vec4};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Meshes {
    meshes: Vec<Mesh>
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    name: String,
    material: String
}

impl Mesh {
    fn new(name: String, material: String, vertices: Vec<Vertex>) -> Mesh {
        Mesh {
            name: name,
            material: material,
            vertices: vertices
        }
    }

    fn print(&self) -> () {
        println!("\t\tMesh: {}", self.name);
        println!("\t\t\tmaterial: {}", self.material);
        println!("\t\t\tNum vertices: {}", self.vertices.len());
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
                    Err(e) => return Err(MeshLoadError::ParseError(String::from(e)))
                },
                _ => match parse_vertex_texture_normal(&parts, &positions, &textures, &normals) {
                    Ok(vertex_texture_normal) => vertices.push(Vertex::PositionNormalTexture(vertex_texture_normal)),
                    Err(e) => return Err(MeshLoadError::ParseError(String::from(e)))
                }
            }
        }
    }

    Ok(vertices)
}

impl Meshes {
    pub fn load(file: &str) -> MeshLoadResult {
        use std::time::SystemTime;
        let now = SystemTime::now();

        let mut material_files: Vec<String> = Vec::new();
        let mut current_material: String = "unknown material".to_owned();
        let mut faces: Vec<Vertex> = Vec::new();
        let mut vertex_normals: Vec<Vec3> = Vec::new();
        let mut vertex_textures: Vec<Vec3> = Vec::new();
        let mut vertices: Vec<Vec4> = Vec::new();
        let mut meshes: Vec<Mesh> = Vec::new();

        let file = File::open(&file).expect("file not found");
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
                "mtllib" => material_files.push(String::from(rest[0])),
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
                    let group_name = String::from(rest[0]);
                    if faces.len() == 0 { continue; }

                    meshes.push(Mesh::new(group_name, current_material.clone(), faces.clone()));
                    faces.clear();   
                },
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

        match now.elapsed() {
            Ok(elapsed) => {
                println!("Time taken to parse: {} seconds", elapsed.as_secs());
            }
            Err(e) => {
                println!("Failed to get timer: {:?}", e);
            }
        }

        Ok(Meshes { meshes: meshes })
    }

    pub fn print(&self) -> () {
        println!("Total meshes: {}", self.meshes.len());

        for mesh in self.meshes.iter() {
            mesh.print();
        }
    }
}
