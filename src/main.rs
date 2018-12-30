#[macro_use]
extern crate glium;
extern crate image;

pub mod io;

mod graphics;
use graphics::shapes::create_billboard;
use graphics::shader::create_shader;
use graphics::textures::{load_texture,load_texture_srgb};

pub mod math;
use math::matrix::Matrix;
use math::constants::FOV;
mod game;
use game::Game;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut game = Game::new(&events_loop);

    let shape = create_billboard(&game);

    let diffuse_texture = load_texture_srgb(&game, "./content/tuto-14-diffuse.jpg", image::JPEG);
    let normal_map = load_texture(&game, "./content/tuto-14-normal.png", image::PNG);

    let program = create_shader("./content/vertex_shader.glsl", "./content/fragment_shader.glsl", &game);

    let model: Matrix = Matrix::identity();
    let view: Matrix = Matrix::view(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);

    use glium::Surface;
    while game.is_running() {
        let mut target = game.get_target();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let perspective: Matrix = Matrix::perspective(target.get_dimensions(), FOV, (0.1f32,1024.0f32));

        let light = [1.4, 0.4, 0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
                    &uniform! { model: model.to_array(),
                                view: view.to_array(),
                                perspective: perspective.to_array(),
                                u_light: light, diffuse_tex: &diffuse_texture, normal_tex: &normal_map },
                    &params).unwrap();
        target.finish().unwrap();

        game = game.run_frame(&mut events_loop);
    }
}