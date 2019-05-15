use std::collections::HashSet;
use sdl2::keyboard::Keycode;
use super::ui::Change;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Cell {
    Alive,
    Dead,
}

pub struct Model {
    grid: Vec<Vec<Cell>>,
    selected_tile: (usize, usize)
}

impl Model {
    pub fn new(w: usize, h: usize) -> Model {
        Model {
            grid: vec![vec![Cell::Dead; w]; h],
            selected_tile: (w/2, h/2)
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        match &self.grid[y][x] {
            Cell::Alive => 1,
            Cell::Dead => 0
        }
    }

    pub fn set<'a>(&mut self, x: usize, y: usize, state: &'a Cell, select: bool) -> HashSet<Change<'a>> {
        let mut res = HashSet::new();
        res.insert(Change::new(x, y, &state, select));
        self.grid[y][x] = *state;
        res
    }

    fn get_neighbors(&self, x: usize, y: usize) -> u8 {
        self.get(x-1, y-1) + self.get(x, y-1) + self.get(x+1, y-1) +
            self.get(x-1, y) +                    self.get(x+1, y) +
            self.get(x-1, y+1) + self.get(x, y+1) + self.get(x+1, y+1)
    }

    pub fn update(&mut self) -> HashSet<Change> {
        let mut new_grid = self.grid.clone();
        let mut res = HashSet::new();
        for j in 1..self.grid.len()-1 {
            for i in 1..self.grid[j].len()-1 {
                match self.get_neighbors(i, j) {
                    2 => {}
                    3 => if self.grid[j][i] == Cell::Dead {
                        new_grid[j][i] = Cell::Alive;
                        res.insert(Change::new(i, j, &Cell::Alive, false));
                    }
                    _ => if self.grid[j][i] == Cell::Alive {
                        new_grid[j][i] = Cell::Dead;
                        res.insert(Change::new(i, j, &Cell::Dead, false));
                    }
                };
            }
        }
        self.grid = new_grid;
        res
    }

    pub fn offset_selected(&mut self, key: Keycode) -> HashSet<Change> {
        let (mut x, mut y) = self.selected_tile;
        let mut res = HashSet::new();
        res.insert(Change::new(x, y, &self.grid[y][x], false));
        match key {
            Keycode::Up => if y > 0 { y-=1 },
            Keycode::Down => if y < self.grid.len()-1 { y+=1 },
            Keycode::Left => if x > 0 { x-=1 },
            Keycode::Right => if x < self.grid[y].len()-1 { x+=1 },
            _ => {}
        }
        self.selected_tile = (x, y);
        res.insert(Change::new(x, y, &self.grid[y][x], true));
        res
    }

    pub fn move_selected(&mut self, x: usize, y: usize) -> HashSet<Change> {
        let (x_old, y_old) = self.selected_tile;
        let mut res = HashSet::new();
        res.insert(Change::new(x_old,y_old, &self.grid[y_old][x_old], false));
        self.selected_tile = (x, y);
        res.insert(Change::new(x, y, &self.grid[y][x], true));
        res
    }

    pub fn flip_selected(&mut self) -> HashSet<Change> {
        let (x, y) = self.selected_tile;
        match self.grid[y][x] {
            Cell::Dead => self.set(x, y, &Cell::Alive, true),
            Cell::Alive => self.set(x, y, &Cell::Dead, true)
        }
    }
}