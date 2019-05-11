extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::collections::HashSet;
use crate::model::Cell;

pub struct UI {
    canvas: WindowCanvas,
    unit: u32
}

impl UI {
    pub fn new(canvas: WindowCanvas, unit: u32) -> UI {
        UI { canvas, unit }
    }

    fn to_screen_space(&self, n: usize) -> i32 {
        (self.unit*n as u32) as i32
    }

    fn render_cell(&mut self, x: usize, y: usize, state: &Cell) {
        //println!("Rendering cell ({}, {})", x, y);
        match state {
            Cell::Alive => self.canvas.set_draw_color(Color::RGB(255, 255, 255)),
            Cell::Dead => self.canvas.set_draw_color(Color::RGB(0, 0 ,0))
        }
        self.canvas.fill_rect(Rect::new(
            self.to_screen_space(x),
            self.to_screen_space(y),
            self.unit,
            self.unit
        ));
    }

    pub fn render(&mut self, changes: HashSet<(usize, usize, Cell)>) {
        for (x, y, cell) in changes.iter() {
            self.render_cell(*x, *y, cell);
        }
        self.canvas.present();
    }
}