mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Renderer {
    iteration: i32,
    height: i32,
    width: i32,
    grid: Vec<Cell>,
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            iteration: 0,
            height: 12,
            width: 12,
            grid: vec![
                //           BLINKER
                Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                //                                                                                  TOAD
                Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                //           BEACON
                Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
                Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead,
            ],
        }
    }
    
    pub fn render(&mut self) -> String {
        self.update();
        self.to_pre().to_string()
    }

    fn update(&mut self) {
        let old = self.grid.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let posn = self.coord_to_posn(row, col) as usize;
                self.grid[posn] = self.new_coord(&old, row, col)
            }
        }
        self.iteration = self.iteration + 1;
    }

    fn coord_to_posn(&self, row: i32, col: i32) -> i32 {
        row*self.width + col
    }

    // fake infinite world by wrapping
    fn new_coord(&self, old: &Vec<Cell>, row: i32, col: i32) -> Cell {
        let alive_adjacent_cells = vec![
            // go thru the eight coord clockwise
            old[self.coord_to_posn(self.vertical(row,  1), col) as usize],                      // 1 down
            old[self.coord_to_posn(self.vertical(row,  1), self.horizontal(col,  1)) as usize], // 2 bottom right
            old[self.coord_to_posn(row,                    self.horizontal(col,  1)) as usize], // 3 right
            old[self.coord_to_posn(self.vertical(row, -1), self.horizontal(col,  1)) as usize], // 4 upper right
            old[self.coord_to_posn(self.vertical(row, -1), col) as usize],                      // 5 up
            old[self.coord_to_posn(self.vertical(row, -1), self.horizontal(col, -1)) as usize], // 6 upper left
            old[self.coord_to_posn(row,                    self.horizontal(col, -1)) as usize], // 7 left
            old[self.coord_to_posn(self.vertical(row,  1), self.horizontal(col, -1)) as usize], // 8 bottom left
            ]
            .iter()
            .filter(|&cell_state| match  cell_state {
                Cell::Alive => true,
                Cell::Dead => false,
            })
            .count();
        return new_coord_impl(old[self.coord_to_posn(row, col) as usize], alive_adjacent_cells)
    }

    fn vertical(&self, row: i32, direction: i32) -> i32 {
        let row = row + direction;
        if row >= self.height {
            0
        } else if row < 0 {
            self.height -1
        } else {
            row
        }
    }

    fn horizontal(&self, col: i32, direction: i32) -> i32 {
        let col = col + direction;
        if col >= self.width {
            0
        } else if col < 0 {
            self.width -1
        } else {
            col
        }
    }

    fn to_pre(&self) -> String {
        let mut result = String::from("");
        for row in 0..self.height {
            for col in 0..self.width {
                let cell_state = self.grid[self.coord_to_posn(row, col) as usize];
                match cell_state {
                    Cell::Alive => result.push_str("*"),
                    Cell::Dead => result.push_str(" "),
                }
            }
            result.push_str("\n");
        }
        result.push_str(self.iteration.to_string().as_str());
        result
    }
}

// Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
// Any live cell with two or three live neighbours lives on to the next generation.
// Any live cell with more than three live neighbours dies, as if by overpopulation.
// Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
fn new_coord_impl(cell: Cell, count: usize) -> Cell {
    match cell {
        Cell::Alive => {
            if count < 2 {
                Cell::Dead
            } else if count >=2 && count <= 3 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }
        Cell::Dead => {
            if count == 3 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }
    }
}