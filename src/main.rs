extern crate sdl2;

use std::env;

mod app;
mod settings;

fn main() {
    match settings::from_args(env::args()) {
        Ok(settings) => {
            let sdl = sdl2::init().expect("Failed to initialize SDL2.");
            let events = sdl.event_pump().expect("Failed to create the event pump.");
            let video_subsystem = sdl.video().expect("Failed to initialize video subsystem.");
            let window = video_subsystem
                .window("LYCHEN : A Game of Life",
                        settings.get("square-size0").unwrap()*settings.get("model-size0").unwrap(),
                        settings.get("square-size0").unwrap()*settings.get("model-size1").unwrap(),
                )
                .opengl()
                .resizable()
                .build()
                .expect("Failed to build the window.");

            match app::App::new(settings, window, events) {
                Ok(mut app) => app.run(),
                Err(e)  => panic!(e)
            }
        },
        Err(e) => settings::print_help(e)
    }
}