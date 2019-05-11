use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Cell {
    Alive,
    Dead
}

pub struct Model {
    grid: Vec<Vec<Cell>>,
}

impl Model {
    pub fn new(w: usize, h: usize) -> Model {
        Model {
            grid: vec![vec![Cell::Dead; w]; h],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        match &self.grid[y][x] {
            Cell::Alive => 1,
            Cell::Dead => 0
        }
    }

    pub fn set(&mut self, x: usize, y: usize, state: Cell) {
        self.grid[y][x] = state;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> u8 {
        self.get(x-1, y-1) + self.get(x, y-1) + self.get(x+1, y-1) +
            self.get(x-1, y) +                    self.get(x+1, y) +
            self.get(x-1, y+1) + self.get(x, y+1) + self.get(x+1, y+1)
    }

    pub fn update(&mut self) -> HashSet<(usize, usize, Cell)> {
        let mut new_grid = self.grid.clone();
        let mut res = HashSet::new();
        for j in 1..self.grid.len()-1 {
            for i in 1..self.grid[j].len()-1 {
                match self.get_neighbors(i, j) {
                    2 => {}
                    3 => if self.grid[j][i] == Cell::Dead {
                        //println!("Cell ({}, {}) is born", i, j);
                        new_grid[j][i] = Cell::Alive;
                        res.insert((i, j, Cell::Alive));
                    }
                    _ => if self.get(i, j) == 1 {
                        //println!("Cell ({}, {}) died", i, j);
                        new_grid[j][i] = Cell::Dead;
                        res.insert((i, j, Cell::Dead));
                    }
                };
            }
        }
        self.grid = new_grid;
        res
    }

}