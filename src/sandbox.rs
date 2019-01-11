use glium::{
    draw_parameters::DepthTest,
    index::{NoIndices, PrimitiveType},
    texture::{SrgbTexture2d, Texture2d},
    Frame, Program, VertexBuffer,
};

use rust_game_engine::{
    game::{Game},
    math::{Mat4x4, Matrix, FOV, Vec3},
    graphics::{create_shader,load_texture, TextureConvert, create_billboard, VertexPositionNormalTexture},
    engine::{create_engine,run}
};

struct DemoGame {
    shape: VertexBuffer<VertexPositionNormalTexture>,
    diffuse_texture: SrgbTexture2d,
    normal_map: Texture2d,
    program: Program,
    model: Mat4x4,
    view: Mat4x4,
}

impl Game for DemoGame {
    fn on_frame(self, frame: &mut Frame) -> DemoGame {
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

fn create_demo_game(display: &glium::Display) -> DemoGame {
    DemoGame {
        shape: create_billboard(&display),
        diffuse_texture: load_texture("./content/tuto-14-diffuse.jpg").as_srgb_texture_2d(&display),
        normal_map: load_texture("./content/tuto-14-normal.png").as_texture_2d(&display),
        program: create_shader(
            "./content/vertex_shader.glsl",
            "./content/fragment_shader.glsl",
            &display,
        ),
        model: Mat4x4::identity(),
        view: Mat4x4::view(
            &[0.5, 0.2, -3.0f32],
            &[-0.5, -0.2, 3.0f32],
            &[0.0, 1.0, 0.0f32]
        ),
    }
}

pub fn run_sandbox(title: &str) {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let display = create_engine(&events_loop, title);
    let game = create_demo_game(&display);
    
    run(&display, &mut events_loop, game);
}
