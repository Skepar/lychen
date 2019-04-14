extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use std::collections::HashSet;

struct Game {
    unit: u32,
    grid: Vec<Vec<u8>>,
    changes: HashSet<(usize, usize)>
}

impl Game {
    fn new(w: usize, h: usize, unit: u32) -> Game {
        Game {
            unit: unit,
            grid: vec![vec![0; w as usize]; h as usize],
            changes: HashSet::new()
        }
    }

    fn render_cell(&self, x: usize, y: usize, canvas: &mut WindowCanvas) {
        //println!("Rendering cell ({}, {})", x, y);
        if self.grid[y][x] == 1 {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
        } else {
            canvas.set_draw_color(Color::RGB(0, 0 ,0));
        }
        canvas.fill_rect(Rect::new(
            (x*self.unit as usize) as i32,
            (y*self.unit as usize) as i32,
            self.unit,
            self.unit
        ));
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        for (x, y) in self.changes.iter() {
            self.render_cell(*x, *y, canvas);
        }
        self.changes.clear();
        canvas.present();
    }

    fn render_all(&self, canvas: &mut WindowCanvas) {
        for j in 0..self.grid.len() {
            for i in 0..self.grid[j].len() {
                self.render_cell(i, j, canvas);
            }
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut res: u8 = 0;
        for j in -1isize..2 {
            for i in -1isize..2 {
                res += self.grid
                    [(j+y as isize) as usize]
                    [(i+x as isize) as usize];
            }
        }
        res - self.grid[y][x]
    }

    fn update(&mut self) {
        for j in 1..self.grid.len()-1 {
            for i in 1..self.grid[j].len()-1 {
                match self.get_neighbors(j, i) {
                    2 => {}
                    3 => if self.grid[j][i] == 0 {
                        //println!("Cell ({}, {}) is born", i, j);
                        self.grid[j][i] = 1;
                        self.changes.insert((i, j));
                    }
                    _ => if self.grid[j][i] == 1 {
                        //println!("Cell ({}, {}) died", i, j);
                        self.grid[j][i] = 0;
                        self.changes.insert((i, j));
                    }
                };
            }
        }
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem.window("Test", 900, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let mut events = sdl.event_pump().unwrap();

    let mut game = Game::new(20, 20, 20);
    game.grid[5][5] = 1;
    game.grid[4][5] = 1;
    game.grid[5][4] = 1;
    game.grid[6][6] = 1;
    game.grid[4][6] = 1;
    game.grid[4][2] = 1;
    game.grid[6][7] = 1;
    game.grid[7][6] = 1;

    game.render_all(&mut canvas);


    // TODO: fix everything (hint: events!)
    loop {
        game.update();
        game.render(&mut canvas);
        game.grid.iter().for_each(|v| println!("{:?}", v));
        std::thread::sleep_ms(10);
        /*match events.wait_event() {
            Event::Quit {..} => break,
            _ =>
        }*/
    }
}