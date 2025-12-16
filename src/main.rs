use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod cell;
mod group;
mod grid;

use crate::grid::Grid;

enum Directional {
    Row,
    Column,
}

struct DirectionalCandidate {
    direction: Directional,
    index: u8,
    value: u8,
}

struct SingleCandidate {
    row: usize,
    column: usize,
    value: u8,
}

struct CellPossibleValues {
    row: usize,
    column: usize,
    value: Vec<u8>,
}

#[derive(Copy, Clone, Default)]
struct PrintCell{
    value: u8,
    number_of_candidates: u8,
    is_given: bool,
}

fn read_sudoku_from_file(path: &str, problem: &mut Grid) {
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
                    problem.set_value(row, col, val, true);
                    
                    println!("***********************************************");
                    problem.print_grid();
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

fn solve_sudoku(problem: &mut Grid) {

    // print grid
    problem.print_grid();

    // Implement Sudoku solving logic here
    // This is a placeholder for the actual solving algorithm
    println!("Solving Sudoku...");

    // create a queue for single candidates
    let mut single_candidate_queue: Vec<SingleCandidate> = Vec::new();
    // create a queue for directional candidates
    let mut directional_candidate_queue: Vec<DirectionalCandidate> = Vec::new();

    // loop through until the puzzle is solved
    let mut count = 0;
    while !problem.is_complete() {
        count += 1;

        problem.print_grid();

        // get candidates
        problem.get_directional_and_single_candidates(&mut directional_candidate_queue, &mut single_candidate_queue);
        // check if both queues are empty
        directional_candidate_queue.clear();
        if single_candidate_queue.is_empty() && directional_candidate_queue.is_empty() {
            println!("No more candidates found, stopping.");
            println!("Total iterations: {}", count);
            // panicking for now
            // panic!("Cannot solve further, no candidates available");
            break;
        }
        // process single candidates
        while let Some(single_candidate) = single_candidate_queue.pop() {
            // println!("Setting single candidate value {} at ({}, {})", single_candidate.value, single_candidate.row, single_candidate.column);
            problem.set_value(single_candidate.row, single_candidate.column, single_candidate.value, false);
        }
        directional_candidate_queue.clear();
        // process directional candidates
        // while let Some(directional_candidate) = directional_candidate_queue.pop() {
        //     match directional_candidate.direction {
        //         Directional::Row => {
        //             for col in 0..9 {
        //                 println!("Setting directional candidate value {} at row {}, column {}", directional_candidate.value, directional_candidate.index, col);
        //                 problem.set_value(directional_candidate.index as usize, col, directional_candidate.value, false);
        //             }
        //         }
        //         Directional::Column => {
        //             for row in 0..9 {
        //                 println!("Setting directional candidate value {} at row {}, column {}", directional_candidate.value, row, directional_candidate.index);
        //                 problem.set_value(row, directional_candidate.index as usize, directional_candidate.value, false);
        //             }
        //         }
        //     }
        // }
    }
    println!("Total iterations to solve: {}", count);
}

fn main() {
    let mut problem = Grid::default();
    let args: Vec<String> = env::args().collect();
    let sudoku_file = "data/".to_string() + if args.len() > 1 {
        &args[1]
    } else {
        "sudoku1.txt"
    };
    read_sudoku_from_file(&sudoku_file, &mut problem);
    solve_sudoku(&mut problem);
    if problem.is_complete() {
        println!("Sudoku solved successfully!");
        println!("************************************************");
        println!("Final Sudoku Grid:");
        problem.print_grid();
        println!("************************************************");
    } else {
        println!("Could not solve the Sudoku completely.");
        let mut remaining_values: Vec<CellPossibleValues> = Vec::new();
        problem.get_remaining_values(&mut remaining_values);
        for cell in remaining_values {
            println!("Cell ({}, {}) possible values: {:?}", cell.row, cell.column, cell.value);
        }
        // problem.print_cell_info(5, 6);
    }
    // You can add printing the grid here to test if it's loaded correctly
    // println!("{:?}", problem);
}
