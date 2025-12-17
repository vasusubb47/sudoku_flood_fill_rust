use std::env;

mod cell;
mod group;
mod grid;
mod utility;

use crate::{grid::Grid, utility::CellPossibleValues};

fn main() {
    let mut problem = Grid::default();
    let args: Vec<String> = env::args().collect();
    let sudoku_file = "data/".to_string() + if args.len() > 1 {
        &args[1]
    } else {
        "sudoku1.txt"
    };
    problem.load_from_file(&sudoku_file);
    let (solved, count) = problem.solve();
    if solved {
        println!("Sudoku solved successfully!");
        println!("************************************************");
        println!("Final Sudoku Grid, after {} iterations:", count);
        problem.print_grid();
        println!("************************************************");
    } else {
        println!("Could not solve the Sudoku completely.");
        let mut remaining_values: Vec<CellPossibleValues> = Vec::new();
        problem.get_remaining_values(&mut remaining_values);
        for cell in remaining_values {
            println!("Cell ({}, {}) possible values: {:?}", cell.row, cell.column, cell.value);
        }
        // problem._print_cell_info(5, 6);
    }
}
