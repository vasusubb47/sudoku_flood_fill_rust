# Sudoku Human-Style Solver

This project is a Rust-based exploration into solving Sudoku puzzles by implementing a constraint propagation algorithm that mimics human-like reasoning. Instead of using brute-force backtracking, it uses a "flood-fill" style approach where setting a value in a single cell triggers a cascade of candidate eliminations across the grid.

## About The Project

The core idea is to represent the Sudoku grid as a hierarchical structure of groups and cells. When a number is placed on the board, the program propagates this information to eliminate that number as a possibility in the corresponding row, column, and 3x3 block. This is achieved through a system of bitmasks that efficiently track available candidates for each cell, row, column, and group.

The code is structured into several modules:
-   `src/main.rs`: Contains the main application logic, including reading a puzzle from a file.
-   `src/cell.rs`: Defines the `Cell` struct, representing a single square on the board.
-   `src/group.rs`: Defines the `Group` struct, representing a 3x3 block of cells.
-   `src/grid.rs`: Defines the `Grid` struct, the top-level structure for the entire 9x9 board.

## How To Run

### Prerequisites
You must have the Rust toolchain (including `rustc` and `cargo`) installed on your system. You can install it from [rustup.rs](https://rustup.rs/).

### Execution
1.  Clone the repository to your local machine.
2.  Navigate to the project directory in your terminal.
3.  Run the application using Cargo:
    ```sh
    cargo run
    ```
4.  The program will automatically load the puzzle from `data/sudoku1.txt`, which triggers the constraint propagation logic. You will see extensive debug output in your terminal showing the process of setting the initial values and eliminating candidates across the grid.

The application is no longer interactive. It demonstrates the file loading and propagation logic.