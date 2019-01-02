extern crate rust_game_engine;
use rust_game_engine::game::Engine;
use rust_game_engine::game::Game;
use rust_game_engine::graphics::shapes::{create_billboard, Vertex};
use rust_game_engine::graphics::{create_shader, load_texture, TextureConvert};
use rust_game_engine::math::matrix::Matrix;
use rust_game_engine::math::FOV;

#[macro_use]
extern crate glium;

struct DemoGame {
    shape: glium::VertexBuffer<Vertex>,
    diffuse_texture: glium::texture::SrgbTexture2d,
    normal_map: glium::texture::Texture2d,
    program: glium::Program,
    model: Matrix,
    view: Matrix,
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
            model: Matrix::identity(),
            view: Matrix::view(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]),
        }
    }
}

impl Game for DemoGame {
    fn on_frame(self, frame: &mut glium::Frame, engine: &Engine) -> DemoGame {
        use glium::Surface;

        let perspective: Matrix =
            Matrix::perspective(frame.get_dimensions(), FOV, (0.1f32, 1024.0f32));

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
    let mut events_loop = glium::glutin::EventsLoop::new();
    let engine = Engine::new(&events_loop);
    let game = DemoGame::new(&engine);

    engine.run(&mut events_loop, game);

    ()
}
