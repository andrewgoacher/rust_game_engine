#[derive(Debug)]

pub enum Face {
    VertexTextureNormal(VTN),
    VertexNormal(VN),
}

#[derive(Debug)]
pub struct VTN {
    v: u32,
    t: u32,
    n: u32,
    smoothing_group: u32,
    material: String,
}

#[derive(Debug)]
pub struct VN {
    v: u32,
    n: u32,
    smoothing_group: u32,
    material: String,
}

#[derive(Debug)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    fn with_w(self, w: f32) -> Vertex {
        Vertex { w: w, ..self }
    }
}

#[derive(Debug)]
pub struct VertexPoint {
    u: f32,
    v: f32,
    w: f32,
}

impl VertexPoint {
    fn new(u: f32, v: f32) -> VertexPoint {
        VertexPoint { u: u, v: v, w: 1.0 }
    }

    fn with_w(self, w: f32) -> VertexPoint {
        VertexPoint { w: w, ..self }
    }
}

#[derive(Debug)]
pub struct VertexNormal {
    i: f32,
    j: f32,
    k: f32,
}

impl VertexNormal {
    fn new(i: f32, j: f32, k: f32) -> VertexNormal {
        VertexNormal { i: i, j: j, k: k }
    }
}

#[derive(Debug)]
pub struct VertexTexture {
    u: f32,
    v: f32,
    w: f32,
}

impl VertexTexture {
    fn new(u: f32, v: f32) -> VertexTexture {
        VertexTexture { u: u, v: v, w: 0.0 }
    }

    fn with_w(self, w: f32) -> VertexTexture {
        VertexTexture { w: w, ..self }
    }
}

#[derive(Debug)]
pub struct ObjectFile {
    materials: Vec<String>,
    vertices: Vec<Vertex>,
    vertex_points: Vec<VertexPoint>,
    vertex_normals: Vec<VertexNormal>,
    vertex_textures: Vec<VertexTexture>,
    faces: Vec<Face>,
    groups: Vec<String>,
    surfaces: Vec<Surface>,
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]

enum Token {
    Vertex(String),
    VertexNormal(String),
    VertexTexture(String),
    VertexPoint(String),
    Face(String),
    Surface(String),
    UseMaterial(String),
    Group(String),
    Unused,
}

fn get_token_inner(token: &str, rest: &str) -> Token {
  match token {
        "#" => Token::Unused,
        "v" => Token::Vertex(String::from(rest)),
        "vn" => Token::VertexNormal(String::from(rest)),
        "vp" => Token::VertexPoint(String::from(rest)),
        "vt" => Token::VertexTexture(String::from(rest)),
        "f" => Token::Face(String::from(rest)),
        "s" => Token::Surface(String::from(rest)),
        "usemtl" => Token::UseMaterial(String::from(rest)),
        "g" => Token::Group(String::from(rest)),
        _ => Token::Unused
    }
}

fn get_token(input: &String) -> Token {
    const r: &str = "^(.+) (.*)$";

    use regex::Regex;
    let re = Regex::new(r).unwrap();
    match re.captures(&input) {
        Some(captures) => {
            let capture_1 = &captures[1];
            let capture = &captures[2];

            get_token_inner(&capture_1, &capture)
        },
        None => Token::Unused
    }
}

fn parse_vertex(line: &String) -> Vertex {
    const vertex_regex: &str = r#"(-?\d+\.?\d?) (-?\d+\.?\d?) (-?\d+\.?\d?) (-?\d+\.?\d?)?"#;
    use regex::Regex;
    let re = Regex::new(vertex_regex).unwrap();

    let captures = re.captures(&line).unwrap();
    let len = captures.len();
    let (x, y, z) = (&captures[1], &captures[2], &captures[3]);

    let vertex = Vertex::new(
        x.parse::<f32>().unwrap(),
        y.parse::<f32>().unwrap(),
        z.parse::<f32>().unwrap(),
    );

    match len {
        4 => vertex.with_w(captures[4].parse::<f32>().unwrap()),
        _ => vertex,
    }
}

fn parse_normals(line: &String) -> VertexNormal {
    const vertex_regex: &str = r#"(-?\d+\.?\d?) (-?\d+\.?\d?) (-?\d+\.?\d?)"#;
    use regex::Regex;
    let re = Regex::new(vertex_regex).unwrap();

    let captures = re.captures(&line).unwrap();
    let len = captures.len();
    let (i, j, k) = (&captures[1], &captures[2], &captures[3]);

    VertexNormal::new(
        i.parse::<f32>().unwrap(),
        j.parse::<f32>().unwrap(),
        k.parse::<f32>().unwrap(),
    )
}

fn parse_points(line: &String) -> VertexPoint {
    const vertex_regex: &str = r#"(-?\d+\.?\d?) (-?\d+\.?\d?) (-?\d+\.?\d?)?"#;
    use regex::Regex;
    let re = Regex::new(vertex_regex).unwrap();

    let captures = re.captures(&line).unwrap();
    let len = captures.len();
    let (u, v) = (&captures[1], &captures[2]);

    let vertex = VertexPoint::new(u.parse::<f32>().unwrap(), v.parse::<f32>().unwrap());

    match len {
        3 => vertex.with_w(captures[3].parse::<f32>().unwrap()),
        _ => vertex,
    }
}

fn parse_textures(line: &String) -> VertexTexture {
    const vertex_regex: &str = r#"(-?\d+\.?\d?) (-?\d+\.?\d?) (-?\d+\.?\d?)?"#;
    use regex::Regex;
    let re = Regex::new(vertex_regex).unwrap();

    let captures = re.captures(&line).unwrap();
    let len = captures.len();
    let (u, v) = (&captures[1], &captures[2]);

    let vertex = VertexTexture::new(u.parse::<f32>().unwrap(), v.parse::<f32>().unwrap());

    match len {
        3 => vertex.with_w(captures[3].parse::<f32>().unwrap()),
        _ => vertex,
    }
}

fn parse_face(line: &String) -> Face {
    panic!("Don't have a face")
}   

fn parse_surface(line: &String) -> Surface {
    match &line[..] {
        "off" => Surface::Off,
        x => Surface::On(x.parse::<u32>().unwrap())
    }
}

#[derive(Debug)]
enum Surface {
    On(u32),
    Off
}

impl ObjectFile {
    pub fn parse(file: &str) -> Option<ObjectFile> {
        let mut materials: Vec<String> = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut points: Vec<VertexPoint> = Vec::new();
        let mut normals: Vec<VertexNormal> = Vec::new();
        let mut textures: Vec<VertexTexture> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();
        let mut groups: Vec<String> = Vec::new();
        let mut surfaces: Vec<Surface> = Vec::new();

        let file = match File::open(&file) {
            Ok(f) => f,
            _ => return None,
        };

        let mut buffer = BufReader::new(&file);

        for line in buffer.lines() {
            let l = line.unwrap();
            match get_token(&l) {
                Token::Vertex(s) => vertices.push(parse_vertex(&s)),
                Token::VertexNormal(s) => normals.push(parse_normals(&s)),
                Token::VertexTexture(s) => textures.push(parse_textures(&s)),
                Token::VertexPoint(s) => points.push(parse_points(&s)),
                // Token::Face(s) => faces.push(parse_face(&s)),
                Token::Group(s) => groups.push(s.clone()),
                Token::UseMaterial(s) => materials.push(s.clone()),
                Token::Surface(s) => surfaces.push(parse_surface(&s)),
                _ => (),
            }
        }

        Some(ObjectFile {
            materials: materials,
            vertices: vertices,
            vertex_points: points,
            vertex_normals: normals,
            vertex_textures: textures,
            faces: faces,
            groups: groups,
            surfaces: surfaces,
        })
    }
}

pub enum Dissolve {
    Factor(f32),
    Halo(f32),
}

pub enum Reflectivity {
    RGB(f32, f32, f32),
    Spectral(String, f32),
    XYZ(f32, f32, f32),
}

pub struct Material {
    name: String,
    // Ns
    specular_component: f32,
    // Ni
    optical_density: f32,
    // d: Tr = 1-d
    dissolve: Dissolve,
    // Tf
    transmission_filter: Reflectivity,
    // illum
    illumination_model: u32,
    // Ka
    ambient_reflectivity: Reflectivity,
    // Kd
    diffuse_reflectivity: Reflectivity,
    // Ks
    specular_reflectivity: Reflectivity,
    // Ke
    emissive_coefficient: Reflectivity,
    map_ka: String,
    map_kd: String,
}
