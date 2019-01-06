use std::io::BufRead;
use std::str::SplitWhitespace;

#[derive(Debug)]
pub struct Meshes {
    vertices: Vec<Vertex>,
}

pub enum MeshLoadError {
    ParseError(String),
    UnknownTokenError(String),
}

pub type MeshLoadResult = Result<Meshes, MeshLoadError>;
type Vec3 = [f32; 3];
type Vec4 = [f32; 4];
type ParseFloat4Result = Result<Vec4, MeshLoadError>;
type ParseFloat3Result = Result<Vec3, MeshLoadError>;
type ParseFaceResult = Result<Vec<Vertex>, MeshLoadError>;

fn parse_float4(parts: &[&str], default: f32) -> ParseFloat4Result {
    let mut result = [default; 4];
    for (i, p) in parts.iter().enumerate() {
        match p.parse::<f32>() {
            Ok(f) => result[i] = f,
            Err(_) => {
                return Err(MeshLoadError::ParseError(String::from(
                    "Couldn't parse floats",
                )))
            }
        };
    }
    Ok(result)
}

fn parse_float3(parts: &[&str], default: f32) -> ParseFloat3Result {
    let mut result = [default; 3];
    for (i, p) in parts.iter().enumerate() {
        match p.parse::<f32>() {
            Ok(f) => result[i] = f,
            Err(_) => {
                return Err(MeshLoadError::ParseError(String::from(
                    "Couldn't parse floats",
                )))
            }
        };
    }
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct VertexPositionNormalTexture {
    position: [f32; 4],
    normal: [f32; 3],
    texture: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct VertexPositionTexture {
    position: [f32; 4],
    texture: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct VertexPositionNormal {
    position: [f32; 4],
    normal: [f32; 3],
}

#[derive(Debug, Clone)]
pub enum Vertex {
    PositionNormalTexture(VertexPositionNormalTexture),
    PositionTexture(VertexPositionTexture),
    PositionNormal(VertexPositionNormal),
}

fn parse_vertex_normal(
    s: &String,
    vertices: &Vec<[f32; 4]>,
    normals: &Vec<[f32; 3]>,
) -> VertexPositionNormal {
    let parts = s.split("//").collect::<Vec<&str>>();
    let vIndex = parts[0].parse::<usize>().unwrap();
    let nIndex = parts[1].parse::<usize>().unwrap();

    VertexPositionNormal {
        position: vertices[vIndex - 1],
        normal: normals[nIndex - 1],
    }
}

fn parse_vertex_texture_normal(
    parts: &[&str],
    vertices: &Vec<Vec4>,
    textures: &Vec<Vec3>,
    normals: &Vec<Vec3>,
) -> VertexPositionNormalTexture {
    let vIndex = parts[0].parse::<usize>().unwrap();
    let tIndex = parts[1].parse::<usize>().unwrap();
    let nIndex = parts[2].parse::<usize>().unwrap();

    VertexPositionNormalTexture {
        position: vertices[vIndex - 1],
        normal: normals[nIndex - 1],
        texture: textures[tIndex - 1],
    }
}

fn parse_vertex_texture(
    parts: &[&str],
    vertices: &Vec<Vec4>,
    textures: &Vec<Vec3>,
) -> VertexPositionTexture {
    let vIndex = parts[0].parse::<usize>().unwrap();
    let tIndex = parts[1].parse::<usize>().unwrap();

    VertexPositionTexture {
        position: vertices[vIndex - 1],
        texture: textures[tIndex - 1],
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
        match s.contains("//") {
            true => vertices.push(Vertex::PositionNormal(parse_vertex_normal(
                &s, &positions, &normals,
            ))),
            false => {
                let parts = s.split("/").collect::<Vec<&str>>();
                match parts.len() {
                    2 => vertices.push(Vertex::PositionTexture(parse_vertex_texture(
                        &parts, &positions, &textures,
                    ))),
                    _ => vertices.push(Vertex::PositionNormalTexture(parse_vertex_texture_normal(
                        &parts, &positions, &textures, &normals,
                    ))),
                };
            }
        };
    }

    Ok(vertices)
}

impl Meshes {
    pub fn load<B>(reader: &mut B) -> MeshLoadResult
    where
        B: BufRead,
    {
        use std::time::SystemTime;
        let now = SystemTime::now();

        let mut material_files: Vec<String> = Vec::new();
        let mut group_name: String = "unknown group".to_owned();
        let mut current_material: String = "unknown material".to_owned();
        let mut faces: Vec<Vertex> = Vec::new();
        let mut vertex_normals: Vec<Vec3> = Vec::new();
        let mut vertex_textures: Vec<[f32; 3]> = Vec::new();
        let mut vertices: Vec<Vec4> = Vec::new();

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
                "g" => group_name = String::from(rest[0]),
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

        Ok(Meshes { vertices: faces })
    }
}
