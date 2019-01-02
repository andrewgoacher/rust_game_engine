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

pub struct VertexPoint {
    u: f32,
    v: f32,
    w: f32,
}

pub struct VertexNormal {
    i: f32,
    j: f32,
    k: f32,
}

pub struct VertexTexture {
    u: f32,
    v: f32,
    w: f32,
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

pub enum Dissolve {
    Factor(f32),
    Halo(f32),
}

pub enum Tf {
    RGB(f32, f32, f32),
    Spectral(String, f32),
    XYZ(f32, f32, f32),
}

pub enum Ka {
    RGB(f32, f32, f32),
    Spectral(String, f32),
    XYZ(f32, f32, f32),
}

pub enum Kd {
    RGB(f32, f32, f32),
    Spectral(String, f32),
    XYZ(f32, f32, f32),
}

pub enum Ks {
    RGB(f32, f32, f32),
    Spectral(String, f32),
    XYZ(f32, f32, f32),
}

pub enum Ke {
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
    transmission_filter: Tf,
    // illum
    illumination_model: u32,
    // Ka
    ambient_reflectivity: Ka,
    // Kd
    diffuse_reflectivity: Kd,
    // Ks
    specular_reflectivity: Ks,
    // Ke
    emissive_coefficient: Ke,
    map_Ka: String,
    map_Kd: String
}
