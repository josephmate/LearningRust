mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Renderer {
    height: u32,
    width: u32,
    grid: Vec<Cell>,
}

#[derive(Copy, Clone)]
enum Cell {
    Alive,
    Dead,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            height: 256,
            width: 256,
            grid: Vec::new(),
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
    }

    fn coord_to_posn(&self, row: u32, col: u32) -> u32 {
        row*self.width + col
    }

    // fake infinite world by wrapping
    fn new_coord(&self, old: &Vec<Cell>, row: u32, col: u32) -> Cell {
        let _adjacent_cells = vec![
            // go thru the eight coord clockwise
            old[self.coord_to_posn(self.vertical(row, 1), col) as usize], // up
            old[self.coord_to_posn(self.vertical(row, 1), self.horizontal(col, 1)) as usize], // upper left
            old[self.coord_to_posn(row,                   self.horizontal(col, 1)) as usize], // left
        ];
        Cell::Alive
    }

    fn vertical(&self, row: u32, direction: u32) -> u32 {
        let row = row + direction;
        if row >= self.height {
            0
        } else if row < 0 {
            self.height -1
        } else {
            row
        }
    }

    fn horizontal(&self, col: u32, direction: u32) -> u32 {
        let col = col + direction;
        if col >= self.width {
            0
        } else if col < 0 {
            self.width -1
        } else {
            col
        }
    }

    // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    // Any live cell with two or three live neighbours lives on to the next generation.
    // Any live cell with more than three live neighbours dies, as if by overpopulation.
    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    fn new_coord_impl(cell: Cell, count: u32) -> Cell {
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

    fn to_pre(&self) -> String {
        String::from("")
    }
}