use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
struct Cell {
    value: Option<u8>,
    is_given: bool,
    candidates: u16,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            value: None,
            is_given: false,
            candidates: 0b11_1111_1110, // All candidates (1-9) are possible
        }
    }
}

impl Cell {
    fn set_value(&mut self, val: u8, is_given: bool) {
        self.value = Some(val);
        self.is_given = is_given;
        self.candidates = 0b0; // Reset to all candidates
    }

    fn can_set_value(&self, val: u8) -> bool {
        // print debug info
        println!("Checking if can set value {}, {:09b}: candidates = {:09b}", val, val, self.candidates);
        (self.candidates & (1 << val)) != 0
    }

    fn _set_candidate(&mut self, val: u8) {
        self.candidates |= 1 << val;
    }

    fn clear_candidate(&mut self, val: u8) {
        self.candidates &= !(1 << val);
        // print debug info
        println!("Cleared candidate {}: now candidates = {:09b}", val, self.candidates);
    }

    fn _set_candidate_val (&mut self, candidates: u16) {
        self.candidates = candidates;
    }

    fn _is_single_candidate(&self) -> bool {
        self.candidates.count_ones() == 1
    }
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
struct Group {
    cells: [[Cell; 3]; 3],
    group_values: u16,
    group_candidates: [[u16; 3]; 2], // row and column candidates for values 1-9
}

impl Default for Group {
    fn default() -> Self {
        Group {
            cells: [[Cell::default(); 3]; 3],
            group_values: 0,
            group_candidates: [[0b11_1111_1110; 3]; 2],
        }
    }
}

impl Group {
    fn set_value(&mut self, row: usize, col: usize, val: u8, is_given: bool) {
        // Ensure the value can be set based on candidates
        if self.cells[row][col].can_set_value(val) {
            // print debug info
            println!("Setting value {} at ({}, {})", val, row, col);
            self.cells[row][col].set_value(val, is_given);
        } else {
            panic!("Cannot set value {} at ({}, {}) due to candidate restrictions", val, row, col);
        }
        self.group_values |= 1 << val;
        self.propagate_changes(val);
    }

    fn propagate_changes(&mut self, val: u8) {
        if (self.group_values & (1 << val)) != 0 {
            // propagate to group candidates
            for idx in 0..3 {
                self.group_candidates[0][idx] &= !(1 << val); // row candidates
                self.group_candidates[1][idx] &= !(1 << val); // column candidates
            }

            // propagate to cells
            for row in 0..3 {
                for col in 0..3 {
                    if self.cells[row][col].value.is_none() {
                        self.cells[row][col].clear_candidate(val);
                    }
                }
            }
        }
    }

    fn recive_propagation(&mut self, row: Option<u8>, col: Option<u8>, val: u8) {
        // propagate to group candidates
        if let Some(col) = col {
            self.group_candidates[1][col as usize] &= !(1 << val); // column candidates
            // propagate to cells in the column
            for row in 0..3 {
                if self.cells[row][col as usize].value.is_none() {
                    self.cells[row][col as usize].clear_candidate(val);
                }
            }
        }
        if let Some(row) = row {
            self.group_candidates[0][row as usize] &= !(1 << val); // row candidates
            // propagate to cells in the row
            for col in 0..3 {
                if self.cells[row as usize][col].value.is_none() {
                    self.cells[row as usize][col].clear_candidate(val);
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
struct Grid {
    sudoku_cells: [[Group; 3]; 3],
    candidates: [[u16; 9]; 2], // row and column candidates for values 1-9
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            sudoku_cells: [[Group::default(); 3]; 3],
            candidates: [[0b11_1111_1110; 9]; 2],
        }
    }
}

impl Grid {
    fn set_value(&mut self, row: usize, column: usize, val: u8, is_given: bool) {
        let grid_row = row / 3;
        let grid_col = column / 3;
        let cell_row = row % 3;
        let cell_col = column % 3;
        // print debug info
        println!("Setting value {} at Grid ({}, {}), Cell ({}, {})", val, grid_row, grid_col, cell_row, cell_col);
        self.sudoku_cells[grid_row][grid_col].set_value(cell_row, cell_col, val, is_given);
        self.propagate_changes(row, column, val);
    }

    fn propagate_changes(&mut self, row: usize, column: usize, val: u8) {
        // Propagate changes to row and column candidates
        let grid_row = row / 3;
        let grid_col = column / 3;
        for idx in 0..3 {
            self.candidates[0][grid_row * 3 + idx] &= !(1 << val); // row candidates
            self.candidates[1][grid_col * 3 + idx] &= !(1 << val); // column candidates
        }
        // Propagate to other groups in the same row
        for col in 0..3 {
            if col != grid_col {
                // print debug info
                println!("Propagating to Group ({}, {}) from value {}", grid_row, col, val);
                self.sudoku_cells[grid_row][col].recive_propagation(Some((row % 3) as u8), None, val);
            }
        }
        // Propagate to other groups in the same column
        for row_idx in 0..3 {
            if row_idx != grid_row {
                // print debug info
                println!("Propagating to Group ({}, {}) from value {}", row_idx, grid_col, val);
                self.sudoku_cells[row_idx][grid_col].recive_propagation(None, Some((column % 3) as u8), val);
            }
        }
    }

}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    let mut problem = Grid::default();
    print!("Enter a cell (row col): ");
    stdin.lock().read_line(&mut line).expect("Failed to read line");
    let inputs: Vec<u8> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    problem.set_value(inputs[0] as usize, inputs[1] as usize, 5, true);
}
