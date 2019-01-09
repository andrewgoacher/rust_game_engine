use glium::{
    draw_parameters::DepthTest,
    index::{NoIndices, PrimitiveType},
    texture::{SrgbTexture2d, Texture2d},
    Frame, Program, VertexBuffer,
};

use rust_game_engine::{
    game::{Engine, Game},
    math::{Mat4x4, Matrix, FOV},
    shapes::{create_billboard, Vertex},
    vector::Vec3,
    graphics::{create_shader,load_texture, TextureConvert}
};

struct DemoGame {
    shape: VertexBuffer<Vertex>,
    diffuse_texture: SrgbTexture2d,
    normal_map: Texture2d,
    program: Program,
    model: Mat4x4,
    view: Mat4x4,
}

impl Game for DemoGame {
    fn on_frame(self, frame: &mut Frame, _engine: &Engine) -> DemoGame {
        use glium::Surface;

        let perspective: Mat4x4 =
            Mat4x4::perspective(frame.get_dimensions(), FOV, (0.1f32, 1024.0f32));

        let light = [1.4, 0.4, 0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        frame
            .draw(
                &self.shape,
                NoIndices(PrimitiveType::TriangleStrip),
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

fn create_demo_game(engine: &Engine) -> DemoGame {
    DemoGame {
        shape: create_billboard(&engine),
        diffuse_texture: load_texture("./content/tuto-14-diffuse.jpg").as_srgb_texture_2d(&engine),
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

pub fn run_sandbox() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let engine = Engine::new(&events_loop);
    let game = create_demo_game(&engine);

    engine.run(&mut events_loop, game);
}
