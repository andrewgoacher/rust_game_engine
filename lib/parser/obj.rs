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

impl ObjectFile {
    fn parse(file: &str) -> ObjectFile {
        let mut materials: Vec<String> = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut points: Vec<VertexPoint> = Vec::new();
        let mut normals: Vec<VertexNormal> = Vec::new();
        let mut textures: Vec<VertexTexture> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();
        let mut groups: Vec<String> = Vec::new();

        ObjectFile {
            materials: materials,
            vertices: vertices,
            vertex_points: points,
            vertex_normals: normals,
            vertex_textures: textures,
            faces: faces,
            groups: groups,
        }
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
