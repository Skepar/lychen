use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use sdl2::mouse::MouseButton;
use std::collections::HashMap;
use sdl2::video::Window;

mod model;
mod ui;

pub struct App {
    model: model::Model,
    ui: ui::UI,
    events: EventPump,
    paused: bool,
    step_time: Duration,
    step_timestamp: Instant
}

impl App {
    pub fn new(settings: HashMap<String, u32>, window: Window, events: EventPump) -> Result<App, String> {
        let canvas = window.into_canvas()
            .accelerated()
            .build()
            .expect("Failed to create canvas.");
        let w: usize = *settings.get("model-size0").unwrap() as usize;
        let h: usize = *settings.get("model-size1").unwrap() as usize;
        let s: u32 = *settings.get("square-size0").unwrap();

        Ok(App {
            model: model::Model::new(w, h),
            ui: ui::UI::new(canvas, s),
            events,
            paused: false,
            step_time: Duration::from_millis(50),
            step_timestamp: Instant::now()
        })
    }

    fn change_step_time(&mut self, dt: i64) {
        if dt > 0 {
            self.step_time += Duration::from_millis(dt as u64);
        } else if Duration::from_millis((-dt) as u64) < self.step_time {
            self.step_time -= Duration::from_millis((-dt) as u64);
        }
    }

    fn handle_keycode(&mut self, key: Keycode) {
        match key {
            Keycode::Space => self.paused = !self.paused,
            Keycode::S => self.change_step_time(5),
            Keycode::F => self.change_step_time(-5),
            Keycode::D => self.step_time = Duration::from_millis(50),
            Keycode::Down | Keycode::Up |
            Keycode::Left | Keycode:: Right => {
                self.paused = true;
                self.ui.render_changes(self.model.offset_selected(key));
            },
            Keycode::Return => {
                self.paused = true;
                self.ui.render_changes(self.model.flip_selected())
            },
            _ => {}
        }
    }

    fn handle_mouse(&mut self, x: i32, y: i32, drawing: bool) {
        if drawing {
            self.ui.render_changes(
                self.model.set(
                    self.ui.to_model_space(x),
                    self.ui.to_model_space(y),
                    &model::Cell::Alive,
                    false
                )
            );
        } else {
            self.ui.render_changes(
                self.model.move_selected(
                    self.ui.to_model_space(x),
                    self.ui.to_model_space(y)
                )
            );
        }
    }

    fn step(&mut self) {
        if !self.paused && (self.step_timestamp.elapsed() >= self.step_time) {
            self.ui.render_changes(self.model.update());
            self.step_timestamp = Instant::now();
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
            // the app's state is refreshed only once every 10ms
            std::thread::sleep(Duration::from_millis(10));
            if let Some(e) = self.events.poll_event() {
                match e {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape),..} => break,
                    Event::KeyDown {keycode: Some(k), repeat: false,..} => self.handle_keycode(k),
                    Event::MouseButtonDown {x, y, mouse_btn: MouseButton::Left,..} => self.handle_mouse(x, y, false),
                    Event::MouseMotion {mousestate, x, y, ..} if mousestate.left() => self.handle_mouse(x, y, true),
                    _ => {}
                }
            }
        }
    }
}