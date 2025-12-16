use serde::{Deserialize, Serialize};
use colored::Colorize;

use crate::{CellPossibleValues, DirectionalCandidate, PrintCell, SingleCandidate, group::Group};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Grid {
    sudoku_cells: [[Group; 3]; 3],
    candidates: [[u16; 9]; 2], // row and column candidates for values 1-9
    completed_groups: [[bool; 3]; 3],
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            sudoku_cells: [[Group::default(); 3]; 3],
            candidates: [[0b11_1111_1110; 9]; 2],
            completed_groups: [[false; 3]; 3],
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
        // println!("Setting value {} at Grid ({}, {}), Cell ({}, {})", val, grid_row, grid_col, cell_row, cell_col);
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
                // println!("Propagating to Group ({}, {}) from value {}", grid_row, col, val);
                self.sudoku_cells[grid_row][col].recive_propagation(Some((row % 3) as u8), None, val);
            }
        }
        // Propagate to other groups in the same column
        for row_idx in 0..3 {
            if row_idx != grid_row {
                // print debug info
                // println!("Propagating to Group ({}, {}) from value {}", row_idx, grid_col, val);
                self.sudoku_cells[row_idx][grid_col].recive_propagation(None, Some((column % 3) as u8), val);
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if !self.sudoku_cells[row][col].is_complete() {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_directional_and_single_candidates(&self, direction_queue: &mut Vec<DirectionalCandidate>, single_queue: &mut Vec<SingleCandidate>) {
        if self.is_complete() {
            return;
        }
        for grid_row in 0..3 {
            for grid_col in 0..3 {
                self.sudoku_cells[grid_row][grid_col].get_single_candidates(single_queue, grid_row, grid_col);
                self.sudoku_cells[grid_row][grid_col].get_directional_candidates(direction_queue);
            }
        }
    }

    pub fn print_grid(&self) {
        let mut grid_values  = [[PrintCell::default(); 9]; 9];
        for grid_row in 0..3 {
            for grid_col in 0..3 {
                // get group values using get_group_cell_values
                self.sudoku_cells[grid_row][grid_col].get_group_cell_values(&mut grid_values, grid_row, grid_col);
            }
        }
        // print buffer
        for row in 0..9 {
            if row % 3 == 0 && row != 0 {
                println!("------+-------+------");
            }
            let _chars = ['E', '!', '@', '#', '$', '%', '^', '&', '*', '.'];
            for col in 0..9 {
                if col % 3 == 0 && col != 0 {
                    print!("| ");
                }
                let val = grid_values[row][col];
                if val.value == 0 {
                    print!("{} ", val.number_of_candidates.to_string().strikethrough());
                } else if val.is_given {
                    print!("{} ", val.value.to_string().blue().bold());
                } else {
                    print!("{} ", val.value.to_string().green());
                }
            }
            println!();
        }
        println!();
        println!();
    }

    pub fn get_remaining_values(&self, buffer: &mut Vec<CellPossibleValues>) {
        buffer.clear();
        for grid_row in 0..3 {
            for grid_col in 0..3 {
                // print debug info
                println!("Getting remaining values for grid ({}, {})", grid_row, grid_col);
                self.sudoku_cells[grid_row][grid_col].get_remaining_values(buffer, grid_row, grid_col);
            }
        }
    }

    pub fn print_cell_info(&self, grid_row: usize, grid_col: usize) {
        self.sudoku_cells[grid_row / 3][grid_col / 3].print_cell_info(grid_row, grid_col);
    }

}
