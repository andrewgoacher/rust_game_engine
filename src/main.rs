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

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let shape = create_billboard(&display);

    let diffuse_texture = load_texture_srgb(&display, "./content/tuto-14-diffuse.jpg", image::JPEG);
    let normal_map = load_texture(&display, "./content/tuto-14-normal.png", image::PNG);

    let program = create_shader("./content/vertex_shader.glsl", "./content/fragment_shader.glsl", &display);

    let mut closed = false;

    let model: Matrix = Matrix::identity();
    let view: Matrix = Matrix::view(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);
    const FOV: f32 = 3.141592 / 3.0;

    while !closed {
        let mut target = display.draw();
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

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }
}