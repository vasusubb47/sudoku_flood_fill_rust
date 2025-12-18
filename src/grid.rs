use std::{fs::File, io::{BufRead, BufReader}};

use serde::{Deserialize, Serialize};
use colored::Colorize;

use crate::{group::Group, utility::{CellPossibleValues, Directional, DirectionalCandidate, PrintCell, SingleCandidate}};

// use crate::utility::{CellPossibleValues, Directional, DirectionalCandidate, PrintCell, SingleCandidate, group::Group};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Grid {
    sudoku_group: [[Group; 3]; 3],
    candidates: [[u16; 9]; 2], // row and column candidates for values 1-9
    completed_groups: [[bool; 3]; 3],
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            sudoku_group: [[Group::default(); 3]; 3],
            candidates: [[0b11_1111_1110; 9]; 2],
            completed_groups: [[false; 3]; 3],
        }
    }
}

impl Grid {

    pub fn solve(&mut self) -> (bool, u8) {

        // print grid
        // self.print_grid();
        println!("Solving Sudoku...");

        // create a queue for single candidates and directional candidates
        let mut single_candidate_queue: Vec<SingleCandidate> = Vec::new();
        let mut directional_candidate_queue: Vec<DirectionalCandidate> = Vec::new();

        // loop through until the puzzle is solved or no more candidates are found
        let mut count = 0;
        while !self.is_complete() {
            count += 1;
            if count > 30 {
                println!("Reached maximum iterations, stopping.");
                break;
            }

            // self.print_grid();

            // get candidates
            // self.get_directional_and_single_candidates(&mut directional_candidate_queue, &mut single_candidate_queue);
            self.get_single_candidates(&mut single_candidate_queue);
            self.get_single_candidate_possible_cell_value(&mut single_candidate_queue);

            if single_candidate_queue.is_empty() {
                println!("No single candidates found in iteration {}", count);
                self.get_directional_candidates(&mut directional_candidate_queue);
            }

            // check if both queues are empty
            if single_candidate_queue.is_empty() && directional_candidate_queue.is_empty() {
                println!("No more candidates found, stopping.");
                println!("Total iterations: {}", count);
                break;
            }
            // process single candidates
            while let Some(single_candidate) = single_candidate_queue.pop() {
                // println!("Setting single candidate value {} at ({}, {})", single_candidate.value, single_candidate.row, single_candidate.column);
                self.set_value(single_candidate.row, single_candidate.column, single_candidate.value, false);
            }
            directional_candidate_queue.clear();
            self.print_grid();
            self.get_directional_candidates(&mut directional_candidate_queue);
            println!("Directional candidates found in iteration {}: {}", count, directional_candidate_queue.len());
            for dc in &directional_candidate_queue {
                println!("  {:?}", dc);
            }
            // process directional candidates
            while let Some(directional_candidate) = directional_candidate_queue.pop() {
                // progate value to the directional groups
                for row in 0..3 {
                    for col in 0..3 {
                        if directional_candidate.grid_col == col && directional_candidate.grid_row == row {
                            continue;
                        }
                        match directional_candidate.direction {
                            Directional::Row => {
                                if row != directional_candidate.grid_row {
                                    continue;
                                }
                            },
                            Directional::Column => {
                                if col != directional_candidate.grid_col {
                                    continue;
                                }
                            },
                        };
                        println!("Propagating directional {:?} to group ({}, {}), value {}", directional_candidate.direction, row, col, directional_candidate.value);
                        self.sudoku_group[row][col].recive_directional_propagation(&directional_candidate);
                    }
                }
            }
            
            // print debug info
            self.print_grid();
            // panic!("terminate after directional propagation for debug");
        }
        println!("Total iterations to solve: {}", count);
        (self.is_complete(), count)
    }

    pub fn load_from_file(&mut self, path: &str) {
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut row = 0;
        for line in reader.lines() {
            if row >= 9 {
                break;
            }
            let line = line.expect("Failed to read line");
            // print debug info
            println!("Reading line {}: {}", row + 1, line);
            if line.trim().is_empty() {
                continue;
            }

            let mut col = 0;
            for c in line.chars() {
                // print debug info
                // println!("Processing character: {}", c);
                if c.is_digit(10) {
                    if col >= 9 {
                        break;
                    }
                    let val = c.to_digit(10).unwrap() as u8;
                    // print debug info
                    // println!("Setting cell at ({}, {}) to value {}", row, col, val);
                    if val != 0 {
                        // print debug info
                        // println!("Setting given value {} at ({}, {})", val, row, col);
                        self.set_value(row, col, val, true);
                        
                        println!("***********************************************");
                        self.print_grid();
                        println!("***********************************************");
                    }
                    col += 1;
                }
            }
            if col > 0 {
                row += 1;
            }
        }
    }

    fn set_value(&mut self, row: usize, column: usize, val: u8, is_given: bool) {
        let grid_row = row / 3;
        let grid_col = column / 3;
        let cell_row = row % 3;
        let cell_col = column % 3;
        // print debug info
        // println!("Setting value {} at Grid ({}, {}), Cell ({}, {})", val, grid_row, grid_col, cell_row, cell_col);
        self.sudoku_group[grid_row][grid_col].set_value(cell_row, cell_col, val, is_given);
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
                // println!("Propagating to Group ({}, {}) from value {}", grid_row, col, val);
                self.sudoku_group[grid_row][col].recive_propagation(Some((row % 3) as u8), None, val);
            }
        }
        // Propagate to other groups in the same column
        for row_idx in 0..3 {
            if row_idx != grid_row {
                // print debug info
                // println!("Propagating to Group ({}, {}) from value {}", row_idx, grid_col, val);
                self.sudoku_group[row_idx][grid_col].recive_propagation(None, Some((column % 3) as u8), val);
            }
        }
    }

    fn is_complete(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if !self.sudoku_group[row][col].is_complete() {
                    return false;
                }
            }
        }
        true
    }

    fn get_single_candidates(&self, queue: &mut Vec<SingleCandidate>) {
        if !queue.is_empty(){
            let msg = format!("{}\n{} ({}) and ({}) in reverse order",
             "Single candidate queue should be empty before getting single candidates.",
             "tips dont call", 
             "get_single_candidate_possible_cell_value".to_string().bold(), 
             "get_single_candidates".to_string().bold()
            );
            panic!("{}", msg);
        }
        if self.is_complete() {
            return;
        }
        for grid_row in 0..3 {
            for grid_col in 0..3 {
                self.sudoku_group[grid_row][grid_col].get_single_candidates(queue, grid_row, grid_col);
            }
        }
    }

    fn get_single_candidate_possible_cell_value(&self, queue: &mut Vec<SingleCandidate>) {
        for grid_row in 0..3 {
            for grid_col in 0..3 {
                self.sudoku_group[grid_row][grid_col].get_single_candidate_possible_cell_value(queue, grid_row, grid_col);
            }
        }
    }

    fn get_directional_candidates(&self, queue: &mut Vec<DirectionalCandidate>) {
        for grid_row in 0..3 {
            for grid_col in 0..3 {
                self.sudoku_group[grid_row][grid_col].get_directional_candidates(queue, grid_row, grid_col);
            }
        }
    }

    pub fn print_grid(&self) {
        let mut grid_values  = [[PrintCell::default(); 9]; 9];
        for grid_row in 0..3 {
            for grid_col in 0..3 {
                // get group values using get_group_cell_values
                self.sudoku_group[grid_row][grid_col].get_group_cell_values(&mut grid_values, grid_row, grid_col);
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
                self.sudoku_group[grid_row][grid_col].get_remaining_values(buffer, grid_row, grid_col);
            }
        }
    }

    pub fn _print_cell_info(&self, grid_row: usize, grid_col: usize) {
        self.sudoku_group[grid_row / 3][grid_col / 3]._print_cell_info(grid_row, grid_col);
    }

}
