use serde::{Deserialize, Serialize};

use crate::cell::Cell;

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Group {
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
    pub fn set_value(&mut self, row: usize, col: usize, val: u8, is_given: bool) {
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

    pub fn propagate_changes(&mut self, val: u8) {
        if (self.group_values & (1 << val)) != 0 {
            // propagate to group candidates
            for idx in 0..3 {
                self.group_candidates[0][idx] &= !(1 << val); // row candidates
                self.group_candidates[1][idx] &= !(1 << val); // column candidates
            }

            // propagate to cells
            for row in 0..3 {
                for col in 0..3 {
                    if self.cells[row][col].has_value() {
                        self.cells[row][col].clear_candidate(val);
                    }
                }
            }
        }
    }

    pub fn recive_propagation(&mut self, row: Option<u8>, col: Option<u8>, val: u8) {
        // propagate to group candidates
        if let Some(col) = col {
            self.group_candidates[1][col as usize] &= !(1 << val); // column candidates
            // propagate to cells in the column
            for row in 0..3 {
                if self.cells[row][col as usize].has_value() {
                    self.cells[row][col as usize].clear_candidate(val);
                }
            }
        }
        if let Some(row) = row {
            self.group_candidates[0][row as usize] &= !(1 << val); // row candidates
            // propagate to cells in the row
            for col in 0..3 {
                if self.cells[row as usize][col].has_value() {
                    self.cells[row as usize][col].clear_candidate(val);
                }
            }
        }
    }
}