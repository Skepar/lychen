extern crate sdl2;

use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use crate::model::Cell;

mod model;
mod ui;

struct Game {
    model: model::Model,
    ui: ui::UI,
    events: EventPump,
    paused: bool,
    step_rate: Duration,
    cell_timestamp: Instant
}

impl Game {
    fn new(w: usize, h: usize, unit: u32, canvas: WindowCanvas, events: EventPump) -> Game {
        Game {
            model: model::Model::new(w, h),
            ui: ui::UI::new(canvas, unit),
            events,
            paused: false,
            step_rate: Duration::from_millis(100),
            cell_timestamp: Instant::now()
        }
    }

    fn handle_keycode(&mut self, key: Keycode) {
        match key {
            Keycode::Space => self.paused = !self.paused,
            Keycode::S => self.step_rate+=Duration::from_millis(10),
            Keycode::F => self.step_rate-=Duration::from_millis(10),
            Keycode::D => self.step_rate = Duration::from_millis(100),
            _ => {}
        }
    }

    fn step(&mut self) {
        if !self.paused && (self.cell_timestamp.elapsed() >= self.step_rate) {
            self.ui.render(self.model.update());
            self.cell_timestamp = Instant::now();
        }
    }

    fn run(&mut self) {
        loop {
            self.step();
            std::thread::sleep(Duration::from_millis(10));
            if let Some(e) = self.events.poll_event() {
                match e {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape),..} => break,
                    Event::KeyDown {keycode: Some(k), repeat: false,..} => self.handle_keycode(k),
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let unit = 10;
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem.window("Test", unit*30, unit*20)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut game = Game::new(
        30,
        20,
        unit,
        window.into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap(),
        sdl.event_pump()
            .unwrap());

    game.model.set(5, 5, Cell::Alive);
    game.model.set(6, 5, Cell::Alive);
    game.model.set(5, 6, Cell::Alive);
    game.model.set(8, 6, Cell::Alive);
    game.model.set(7, 5, Cell::Alive);
    game.model.set(7, 7, Cell::Alive);
    game.model.set(8, 6, Cell::Alive);

    game.run();
}