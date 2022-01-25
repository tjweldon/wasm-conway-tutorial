mod utils;

extern crate rand;

use wasm_bindgen::prelude::*;
use std::fmt;
use std::fmt::Formatter;
use rand::thread_rng;
use rand::Rng;
use std::collections::VecDeque;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}


impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    state_deque: VecDeque<Vec<Cell>>,
    changes: Vec<CellDelta>,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellDelta {
    Birth = 1,
    None = 2,
    Death = 0,
}


impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_cell(&self, row: u32, column: u32) -> Cell {
        self.cells[self.get_index(row, column)]
    }

    fn get_cell_byte(&self, row: u32, column: u32) -> u8 {
        self.get_cell(row, column) as u8
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                count += self.get_cell_byte(neighbor_row, neighbor_col);
            }
        }
        count
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    fn dead_cells(width: u32, height: u32) -> Vec<Cell> {
        (0..width * height)
            .map(|_i| Cell::Dead)
            .collect()
    }

    fn calculate_changes(current: &Vec<Cell>, prev: &Vec<Cell>) -> Vec<CellDelta> {
        let deltas = (0..current.len())
            .map(|i| {
                let item = match (current[i], prev[i]) {
                    (Cell::Dead, Cell::Alive) => CellDelta::Death,
                    (Cell::Alive, Cell::Dead) => CellDelta::Birth,
                    (_x, _y) => CellDelta::None
                };
                item
            })
            .collect()
            ;

        deltas
    }

    fn safe_state_from_deque(&self, deque: VecDeque<Vec<Cell>>, index: usize) -> Vec<Cell> {
        match deque.get(index) {
            Some(cells) => cells.to_vec(),
            None => Universe::dead_cells(self.width, self.height)
        }
    }

    fn get_state_deque(&self) -> VecDeque<Vec<Cell>> {
        self.state_deque.clone()
    }

    fn get_changes(&self) -> Vec<CellDelta> {
        let prev = self.safe_state_from_deque(self.get_state_deque(), 1);
        let current = self.safe_state_from_deque(self.get_state_deque(), 0);
        Universe::calculate_changes(&current, &prev)
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = match self.state_deque.pop_back() {
            Some(cells) => cells,
            None => Universe::dead_cells(self.width, self.height)
        };

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.state_deque.push_front(next.to_vec());
        self.cells = next;

        self.changes = self.get_changes();
    }

    pub fn count_neighbours(&self, row: u32, column: u32) -> u8 {
        self.live_neighbor_count(row, column)
    }

    pub fn new_sized(width: u32, height: u32) -> Universe {
        let mut rng = thread_rng();
        let cells: Vec<Cell> = (0..width * height)
            .map(|_i| {
                let roll: f64 = rng.gen_range(0.0, 100.0);
                if roll < 60.0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let dead_cells = Universe::dead_cells(width, height);
        let current = cells.to_vec();

        let changes = Universe::calculate_changes(&current, &dead_cells);
        let mut state_deque = VecDeque::from([]);
        state_deque.push_front(dead_cells);
        state_deque.push_front(current);


        Universe {
            width,
            height,
            cells,
            state_deque,
            changes,
        }
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        Universe::new_sized(width, height)
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn changes(&self) -> *const CellDelta {
        self.changes.as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}