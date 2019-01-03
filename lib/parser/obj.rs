pub enum Face {
    VertexTextureNormal(VTN),
    VertexNormal(VN),
}

pub struct VTN {
    v: u32,
    t: u32,
    n: u32,
    smoothing_group: u32,
    material: String,
}

pub struct VN {
    v: u32,
    n: u32,
    smoothing_group: u32,
    material: String,
}

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

pub struct ObjectFile {
    materials: Vec<String>,
    vertices: Vec<Vertex>,
    vertex_points: Vec<VertexPoint>,
    vertex_normals: Vec<VertexNormal>,
    vertex_textures: Vec<VertexTexture>,
    faces: Vec<Face>,
    groups: Vec<String>,
}

use std::fs::File;
use std::io::{BufReader, BufRead};


enum Token {
    Vertex,
    VertexNormal,
    VertexTexture,
    VertexPoint,
    Face,
    Surface,
    UseMaterial,
    Group,
    Unused
}

fn get_token(input: &String) -> Token {
    const r: &str = "^(.+) *$";

    use regex::Regex;
    let re = Regex::new(r).unwrap();
    let captures = re.captures(&input).unwrap();

    match &captures[0] {
        "#" => Token::Unused,
        "v" => Token::Vertex,
        "vn" => Token::VertexNormal,
        "vp" => Token::VertexPoint,
        "vt" => Token::VertexTexture,
        "f" => Token::Face,
        "s" => Token::Surface,
        "usemtl" => Token::UseMaterial,
        "g" => Token::Group,
        _ => panic!("unknown token")
    }
}

fn parse_vertex(line: &String) -> Vertex {
    Vertex::new(1.0,2.0,3.0)
}

fn parse_normals(line: &String) -> VertexNormal {
    VertexNormal::new(1.0, 2.0, 3.0)
}

fn parse_points(line: &String) -> VertexPoint {
    VertexPoint::new(1.0, 2.0) 
}

impl ObjectFile {
    fn parse(file: &str) -> Option<ObjectFile> {
        let mut materials: Vec<String> = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut points: Vec<VertexPoint> = Vec::new();
        let mut normals: Vec<VertexNormal> = Vec::new();
        let mut textures: Vec<VertexTexture> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();
        let mut groups: Vec<String> = Vec::new();

        let file = match File::open(&file) {
            Ok(f) => f,
            _ => return None
        };

        let mut buffer = BufReader::new(&file);

        for line in buffer.lines() {
            let l = line.unwrap();
            match get_token(&l) {
                Token::Vertex => vertices.push(parse_vertex(&l)),
                Token::VertexNormal => normals.push(parse_normals(&l)),
                Token::VertexPoint => points.push(parse_points(&l)),
                _ => ()
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
