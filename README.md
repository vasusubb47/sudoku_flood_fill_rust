# Sudoku Human-Style Solver

This project is a Rust-based exploration into solving Sudoku puzzles by implementing a constraint propagation algorithm that mimics human-like reasoning. Instead of using brute-force backtracking, it uses a "flood-fill" style approach where setting a value in a single cell triggers a cascade of candidate eliminations across the grid.

## About The Project

The core idea is to represent the Sudoku grid as a hierarchical structure of groups and cells. When a number is placed on the board, the program propagates this information to eliminate that number as a possibility in the corresponding row, column, and 3x3 block. This is achieved through a system of bitmasks that efficiently track available candidates for each cell, row, column, and group.

The code is structured around three main data structures:
-   `Cell`: Represents a single square on the board, tracking its value and a bitmask of possible candidates.
-   `Group`: Represents a 3x3 block of cells.
-   `Grid`: The top-level structure representing the entire 9x9 board.

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
4.  The program will prompt you to enter a cell's coordinates (e.g., `0 0` for the top-left cell). It will then set the value `5` at that position and print debug information showing how candidates are eliminated based on this new value.

This demonstrates the core propagation logic of the solver.
