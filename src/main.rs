extern crate rust_game_engine;
use rust_game_engine::game::Engine;
use rust_game_engine::game::Game;
use rust_game_engine::graphics::{create_shader, load_texture, TextureConvert};
use rust_game_engine::math::FOV;
use rust_game_engine::math::{Mat4x4, Matrix};
use rust_game_engine::shapes::{create_billboard, Vertex};
use rust_game_engine::vector::{Vec3, Vec4, Vector};

use rust_game_engine::mesh::MeshDescriptions;
use rust_game_engine::parser::{Parseable,ParseError};

#[macro_use]
extern crate glium;

struct DemoGame {
    shape: glium::VertexBuffer<Vertex>,
    diffuse_texture: glium::texture::SrgbTexture2d,
    normal_map: glium::texture::Texture2d,
    program: glium::Program,
    model: Mat4x4,
    view: Mat4x4,
}

impl DemoGame {
    fn new(engine: &Engine) -> DemoGame {
        DemoGame {
            shape: create_billboard(&engine),
            diffuse_texture: load_texture("./content/tuto-14-diffuse.jpg")
                .as_srgb_texture_2d(&engine),
            normal_map: load_texture("./content/tuto-14-normal.png").as_texture_2d(&engine),
            program: create_shader(
                "./content/vertex_shader.glsl",
                "./content/fragment_shader.glsl",
                &engine,
            ),
            model: Mat4x4::identity(),
            view: Mat4x4::view(
                &Vec3 {
                    x: 0.5f32,
                    y: 0.2f32,
                    z: -3.0f32,
                },
                &Vec3 {
                    x: -0.5f32,
                    y: -0.2f32,
                    z: 3.0f32,
                },
                &Vec3 {
                    x: 0.0f32,
                    y: 1.0f32,
                    z: 0.0f32,
                },
            ),
        }
    }
}

impl Game for DemoGame {
    fn on_frame(self, frame: &mut glium::Frame, engine: &Engine) -> DemoGame {
        use glium::Surface;

        let perspective: Mat4x4 =
            Mat4x4::perspective(frame.get_dimensions(), FOV, (0.1f32, 1024.0f32));

        let light = [1.4, 0.4, 0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        frame
            .draw(
                &self.shape,
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &self.program,
                &uniform! { model: self.model.to_array(),
                view: self.view.to_array(),
                perspective: perspective.to_array(),
                u_light: light, diffuse_tex: &self.diffuse_texture, normal_tex: &self.normal_map },
                &params,
            ).unwrap();

        DemoGame { ..self }
    }
}

fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    // let mut events_loop = glium::glutin::EventsLoop::new();
    // let engine = Engine::new(&events_loop);
    // let game = DemoGame::new(&engine);

    // engine.run(&mut events_loop, game);

    let mesh = match MeshDescriptions::from_file("./content/Millenium Falcon/millenium-falcon.obj") {
        Ok(m) => m,
        Err(e) => match e {
            ParseError::UnknownToken(err) => panic!("Unknown token: {}", err),
            ParseError::GeneralError(err) => panic!("Parsing error: {}", err),
            _ => panic!("Unresolved mesh load error"),
        },
    };

    println!("mesh \n{}", mesh);

    for (_,mat) in mesh.materials.iter() {
        println!("\t\t{}", mat);
    }

    for mesh in mesh.meshes.iter() {
        println!("\t\t{}", mesh)
    }

    // //    println!("{:?}", o);
    // mesh.print();

    ()
}
