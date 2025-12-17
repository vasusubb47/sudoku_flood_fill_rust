use serde::{Deserialize, Serialize};

use crate::{cell::Cell, utility::{CellPossibleValues, Directional, DirectionalCandidate, PrintCell, SingleCandidate}};

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
        if self.is_complete() {
            panic!("Cannot set value {}, group is already complete", val);
        }
        if (self.group_values & (1 << val)) != 0 {
            panic!("Value {} already set in this group", val);
        }
        // Ensure the value can be set based on candidates
        if self.cells[row][col].can_set_value(val) {
            // print debug info
            // println!("Setting value {} at ({}, {})", val, row, col);
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
                    if !self.cells[row][col].has_value() {
                        // print debug info
                        // println!("Propagating to cell ({}, {}) from value {}", row, col, val);
                        self.cells[row][col].clear_candidate(val);
                    }
                }
            }
        }
    }

    fn get_remaining_group_values(&self) -> Vec<u8> {
        let mut values: Vec<u8> = Vec::new();
        for val in 1..10 {
            if (self.group_values & (1 << val)) == 0 {
                values.push(val as u8);
            }
        }
        values
    }

    pub fn get_single_candidate_possible_cell_value(&self, queue: &mut Vec<SingleCandidate>, grid_row: usize, grid_col: usize) {
        if self.is_complete() {
            return;
        }
        let remaining_values = self.get_remaining_group_values();
        for val in &remaining_values {
            let mut possible_cells: Vec<((usize, usize), u8)> = Vec::new();
            for row in 0..3 {
                for col in 0..3 {
                    if !self.cells[row][col].has_value() {
                        let possible_values = self.cells[row][col].get_possible_values();
                        if possible_values.contains(val) {
                            possible_cells.push(((row, col), *val));
                        }
                    }
                }
            }
            if possible_cells.len() == 1 {
                let ((row, col), value) = possible_cells[0];

                // check if already in queue
                let mut already_in_queue = false;
                for candidate in queue.iter() {
                    if candidate.row == grid_row * 3 + row && candidate.column == grid_col * 3 + col && candidate.value == value {
                        already_in_queue = true;
                        break;
                    }
                }
                if !already_in_queue {
                    queue.push(SingleCandidate {
                        row: grid_row * 3 + row,
                        column: grid_col * 3 + col,
                        value,
                    });
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
                if !self.cells[row][col as usize].has_value() {
                    self.cells[row][col as usize].clear_candidate(val);
                }
            }
        }
        if let Some(row) = row {
            self.group_candidates[0][row as usize] &= !(1 << val); // row candidates
            // propagate to cells in the row
            for col in 0..3 {
                if !self.cells[row as usize][col].has_value() {
                    self.cells[row as usize][col].clear_candidate(val);
                }
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        self.group_values.count_ones() == 9
    }

    pub fn get_directional_candidates(&self, queue: &mut Vec<DirectionalCandidate>, grid_row: usize, grid_col: usize) {
        if self.is_complete() {
            return;
        }
        for val in 1..10 {
            if (self.group_values & (1 << val)) != 0 {
                continue;
            }
            let mut row_count: Vec<u8> = Vec::new();
            let mut col_count: Vec<u8> = Vec::new();
            for idx in 0..3 {
                if (self.group_candidates[0][idx] & (1 << val)) != 0 {
                    row_count.push(idx as u8);
                }
                if (self.group_candidates[1][idx] & (1 << val)) != 0 {
                    col_count.push(idx as u8);
                }
            }
            if row_count.len() == 1 {
                queue.push(DirectionalCandidate {
                    direction: Directional::Row,
                    index: row_count[0] + (grid_row as u8 * 3),
                    grid_row,
                    grid_col,
                    value: val as u8,
                });
            }
            if col_count.len() == 1 {
                queue.push(DirectionalCandidate {
                    direction: Directional::Column,
                    index: col_count[0] + (grid_col as u8 * 3),
                    grid_row,
                    grid_col,
                    value: val as u8,
                });
            }
        }
    }

    pub fn get_single_candidates(&self, queue: &mut Vec<SingleCandidate>, grid_row: usize, grid_col: usize) {
        if self.is_complete() {
            return;
        }
        for row in 0..3 {
            for col in 0..3 {
                if !self.cells[row][col].has_value() {
                    if let Some(val) = self.cells[row][col].is_single_candidate() {
                        queue.push(SingleCandidate {
                            row: grid_row * 3 + row,
                            column: grid_col * 3 + col,
                            value: val,
                        });
                    }
                }
            }
        }
    }

    pub fn get_group_cell_values(&self, buffer: &mut [[PrintCell; 9]; 9], grid_row: usize, grid_col: usize) {
        for row in 0..3 {
            for col in 0..3 {
                buffer[grid_row * 3 + row][grid_col * 3 + col] = match self.cells[row][col].get_value() {
                    Some(val) => PrintCell { value: val, number_of_candidates: 0, is_given: self.cells[row][col].is_given },
                    None => PrintCell { value: 0, number_of_candidates: self.cells[row][col].get_number_of_candidates(), is_given: false },
                };
            }
        }
    }

    pub fn get_remaining_values(&self, buffer: &mut Vec<CellPossibleValues>, grid_row: usize, grid_col: usize) {
        for row in 0..3 {
            for col in 0..3 {
                let cell = &self.cells[row][col];
                if !cell.has_value() {
                    buffer.push(CellPossibleValues {
                        row: grid_row * 3 + row,
                        column: grid_col * 3 + col,
                        value: cell.get_possible_values(),
                    });
                }
            }
        }
    }

    pub fn _print_cell_info(&self, grid_row: usize, grid_col: usize) {
        let row = grid_row % 3;
        let col = grid_col % 3;
        self.cells[row][col]._print_info();
    }
}
