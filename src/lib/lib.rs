#[macro_use]
extern crate glium;
extern crate image;

pub mod game;
pub mod graphics;
pub mod io;
pub mod math;

use graphics::shader::create_shader;
use graphics::shapes::create_billboard;
use graphics::textures::{load_texture, load_texture_srgb};
use math::constants::FOV;
use math::matrix::Matrix;
use game::Engine;


pub fn demo() -> () {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let mut engine = Engine::new(&events_loop);

    let shape = create_billboard(&engine);

    let diffuse_texture = load_texture_srgb(&engine, "./content/tuto-14-diffuse.jpg", image::JPEG);
    let normal_map = load_texture(&engine, "./content/tuto-14-normal.png", image::PNG);

    let program = create_shader(
        "./content/vertex_shader.glsl",
        "./content/fragment_shader.glsl",
        &engine,
    );

    let model: Matrix = Matrix::identity();
    let view: Matrix = Matrix::view(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);

    use glium::Surface;
    while engine.is_running() {
        let mut target = engine.get_target();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let perspective: Matrix =
            Matrix::perspective(target.get_dimensions(), FOV, (0.1f32, 1024.0f32));

        let light = [1.4, 0.4, 0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        target
            .draw(
                &shape,
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &program,
                &uniform! { model: model.to_array(),
                view: view.to_array(),
                perspective: perspective.to_array(),
                u_light: light, diffuse_tex: &diffuse_texture, normal_tex: &normal_map },
                &params,
            ).unwrap();
        target.finish().unwrap();

        engine = engine.run_frame(&mut events_loop);
    }
}
