extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use super::model::Cell;

// a Change represents the evolution on the screen of a cell in the model
#[derive(PartialEq, Eq)]
pub struct Change<'a> {
    x: usize,
    y: usize,
    state: &'a Cell,
    selected: bool
}

impl<'a> Hash for Change<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<'a> Change<'a> {
    pub fn new(x: usize, y: usize, state: &Cell, selected: bool) -> Change {
        Change {x, y, state, selected }
    }
}

pub struct UI {
    canvas: WindowCanvas,
    unit: u32
}

impl UI {
    pub fn new(canvas: WindowCanvas, unit: u32) -> UI {
        UI { canvas, unit }
    }

    // allow us to convert a cell in the model to a point on the screen
    fn to_screen_space(&self, n: usize) -> i32 {
        (self.unit*n as u32) as i32
    }

    // allow us to convert a point on the screen to a cell in the model
    pub fn to_model_space(&self, n: i32) -> usize {
        (n as u32/self.unit) as usize
    }

    fn render_cell(&mut self, x: usize, y: usize, state: &Cell, selected: bool) {
        match state {
            Cell::Alive => if selected {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                self.canvas.set_draw_color(Color::RGB(200, 200, 200));
            },
            Cell::Dead => if selected {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                self.canvas.set_draw_color(Color::RGB(0, 0 ,0));
            }
        }
        self.canvas.fill_rect(Rect::new(
            self.to_screen_space(x),
            self.to_screen_space(y),
            self.unit,
            self.unit
        ));
    }

    pub fn render_changes(&mut self, changes: HashSet<Change>) {
        for change in changes.iter() {
            let Change { x, y, state, selected } = change;
            self.render_cell(*x, *y, state, *selected);
        }
        self.canvas.present();
    }
}