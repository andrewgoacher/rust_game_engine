use std::io::BufRead;
use std::str::SplitWhitespace;

pub struct Meshes {
    vertices: &[VertexPositionNormalTexture]
}

pub enum MeshLoadError {
    GeneralError,
    ParseError(String),
    UnknownTokenError(String),
}

pub type MeshLoadResult = Result<Meshes, MeshLoadError>;
type ParseFloat4Result = Result<[f32; 4], MeshLoadError>;
type ParseFloat3Result = Result<[f32; 3], MeshLoadError>;

fn parse_float4(parts: SplitWhitespace, default: f32) -> ParseFloat4Result {
    let mut result = [default; 4];
    for (i, p) in parts.enumerate() {
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

fn parse_float3(parts: SplitWhitespace, default: f32) -> ParseFloat3Result {
    let mut result = [default; 3];
    for (i, p) in parts.enumerate() {
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

pub struct VertexPositionNormalTexture {
    position: [f32;4],
    normal: [f32;3],
    texture: [f32;3]
}

impl Meshes {
    pub fn load<B>(reader: &mut B) -> MeshLoadResult
    where
        B: BufRead,
    {
        use std::time::SystemTime;
        let now = SystemTime::now();

        let mut material_files: Vec<String> = Vec::new();

        for line in reader.lines() {
            let (line, mut parts) = match line {
                Ok(ref line) => (&line[..], line[..].split_whitespace()),
                Err(e) => return Err(MeshLoadError::ParseError(String::from(format!("{:?}", e)))),
            };

            let mut vertices: Vec<[f32; 4]> = Vec::new();
            let mut vertex_normals: Vec<[f32; 3]> = Vec::new();
            let mut vertex_textures: Vec<[f32;3]> = Vec::new();
            let mut group_name: &str = "unknown group";
            let mut current_material: &str = "unknown material";

            match parts.next() {
                Some("mtllib") => material_files.push(String::from(parts.next().unwrap())),
                Some("v") => {
                    match parse_float4(parts, 1.0f32) {
                        Ok(arr) => vertices.push(arr),
                        Err(e) => return Err(e),
                    };
                },
                Some("vn") => {
                    match parse_float3(parts, 1.0f32) {
                        Ok(arr) => vertex_normals.push(arr),
                        Err(e) => return Err(e),
                    };
                },
                Some("vt") => {
                    match parse_float3(parts, 0.0f32) {
                        Ok(arr) => vertex_textures.push(arr),
                        Err(e) => return Err(e),
                    };
                },
                Some("g") => group_name = parts.next().unwrap(),
                Some("usemtl") => current_material = parts.next().unwrap(),
                Some("f") => 
                Some("vp") => continue,
                Some("s") => continue,
                Some("#") => continue,
                Some(x) => return Err(MeshLoadError::UnknownTokenError(String::from(x))),
                None => continue,
            }
        }

        match now.elapsed() {
            Ok(elapsed) => {
                println!("Time taken to parse: {}", elapsed.as_secs());
            }
            Err(e) => {
                println!("Failed to get timer: {:?}", e);
            }
        }

        Err(MeshLoadError::GeneralError)
    }
}
