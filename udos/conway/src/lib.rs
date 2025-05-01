#![forbid(unsafe_code)]
use std::cmp::min;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default + std::fmt::Debug> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        // TODO: your code goes here.
        Grid {
            rows: rows,
            cols: cols,
            grid: vec![T::default(); rows * cols],
        }
        // unimplemented!()
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        // TODO: your code goes here.
        Grid {
            rows: rows,
            cols: cols,
            grid: Vec::from(grid),
        }
        // unimplemented!()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
        // TODO: your code goes here.
        // unimplemented!()
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        // TODO: your code goes here.
        // unimplemented!()
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        // TODO: your code goes here.
        // unimplemented!()
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        // TODO: your code goes here.
        // unimplemented!()
        let ds = [0, 1, 2];
        let idx = &ds[1 - min(1, col)..2 + min(1, self.cols - col - 1)];
        let idy = &ds[1 - min(1, row)..2 + min(1, self.rows - row - 1)];
        let mut result = Vec::<(usize, usize)>::new();
        for x in idx {
            for y in idy {
                if *x == 1 && *y == 1 {
                    continue;
                }
                result.push((col + x - 1, row + y - 1));
            }
        }
        result
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        // TODO: your code goes here.
        // unimplemented!()
        GameOfLife { grid: grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        // TODO: your code goes here.
        // unimplemented!()
        &self.grid
    }

    pub fn step(&mut self) {
        // TODO: your code goes here.
        // unimplemented!()
        let mut new_grid = Grid::<Cell>::new(self.grid.rows, self.grid.cols);
        for (index, _cell) in self.grid.grid.iter().enumerate() {
            let x = index % self.grid.cols;
            let y = index / self.grid.cols;
            let adjs = self.grid.neighbours(x, y);
            let alive = adjs
                .iter()
                .filter_map(|&(nx, ny)| Some(self.grid.get(nx, ny)))
                .filter(|cell| matches!(cell, Cell::Alive))
                .count();
            let &curr = self.grid.get(y, x);
            let life = match curr {
                Cell::Dead => match alive == 3 {
                    true => Cell::Alive,
                    false => Cell::Dead,
                },
                Cell::Alive => match alive == 2 || alive == 3 {
                    true => Cell::Alive,
                    false => Cell::Dead,
                },
            };
            new_grid.set(life, y, x);
        }
        self.grid = new_grid;
    }
}
