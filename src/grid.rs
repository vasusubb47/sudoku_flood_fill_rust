use serde::{Deserialize, Serialize};

use crate::group::Group;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Grid {
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
    pub fn set_value(&mut self, row: usize, column: usize, val: u8, is_given: bool) {
        let grid_row = row / 3;
        let grid_col = column / 3;
        let cell_row = row % 3;
        let cell_col = column % 3;
        // print debug info
        println!("Setting value {} at Grid ({}, {}), Cell ({}, {})", val, grid_row, grid_col, cell_row, cell_col);
        self.sudoku_cells[grid_row][grid_col].set_value(cell_row, cell_col, val, is_given);
        self.propagate_changes(row, column, val);
    }

    pub fn propagate_changes(&mut self, row: usize, column: usize, val: u8) {
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
