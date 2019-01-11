extern crate clap;
extern crate rust_game_engine;

#[macro_use]
extern crate glium;

mod sandbox;

use clap::{App, Arg};

use rust_game_engine:: {
    graphics::MeshDescriptions,
    parser::{FromFile,ParseError}
};

use sandbox::run_sandbox;

fn main() {
    const TITLE: &str = "Rust Game Engine";

    let matches = App::new(TITLE)
        .arg(
            Arg::with_name("scene")
                .short("s")
                .long("scene")
                .takes_value(true),
        ).get_matches();
    
    match matches.value_of("scene") {
        None => run_scene("sandbox", TITLE),
        Some(s) => run_scene(s, TITLE)
    }
}

fn run_scene(scene: &str, title: &str) {
    match &scene.to_lowercase()[..] {
        "sandbox" => run_sandbox(title),
        "falcon" => load_mesh_scene("./content/Millenium Falcon/millenium-falcon.obj", "Millenium Falcon"),
        "earth" => load_mesh_scene("./content/Earth/earth.obj", "Earth"),
        "ironman" => load_mesh_scene("./content/IronMan/IronMan.obj", "IronMan"),
        _ => println!("unrecognised scene!")
    }
}

fn load_mesh_scene(mesh_file: &str, desc: &str) {
     let mesh = load_mesh(&mesh_file);
            println!("Loaded {}!", &desc);
            println!("{}", &mesh);
}

fn load_mesh(mesh_file: &str) -> MeshDescriptions{
     match MeshDescriptions::from_file(&mesh_file) {
        Ok(m) => m,
        Err(e) => match e {
            ParseError::UnknownToken(err) => panic!("Unknown token: {}", err),
            ParseError::GeneralError(err) => panic!("Parsing error: {}", err),
        },
    }
}
