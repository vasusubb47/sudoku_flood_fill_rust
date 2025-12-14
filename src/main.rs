use std::fs::File;
use std::io::{BufRead, BufReader};

mod cell;
mod group;
mod grid;

use crate::grid::Grid;

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
            println!("Processing character: {}", c);
            if c.is_digit(10) {
                if col >= 9 {
                    break;
                }
                let val = c.to_digit(10).unwrap() as u8;
                // print debug info
                println!("Setting cell at ({}, {}) to value {}", row, col, val);
                if val != 0 {
                    // print debug info
                    println!("Setting given value {} at ({}, {})", val, row, col);
                    problem.set_value(row, col, val, true);
                }
                col += 1;
            }
        }
        if col > 0 {
            row += 1;
        }
    }
}

fn main() {
    let mut problem = Grid::default();
    read_sudoku_from_file("data/sudoku1.txt", &mut problem);
    // You can add printing the grid here to test if it's loaded correctly
    // println!("{:?}", problem);
}
