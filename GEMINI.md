# Sudoku Human-Style Solver Analysis

This document outlines the structure and core logic of the Rust-based Sudoku solver. The implementation uses a hierarchical, "flood-fill" style approach to propagate constraints and solve the puzzle.

## Program Structure

The program represents the Sudoku grid using three nested data structures: `Cell`, `Group`, and `Grid`.

### `Cell`
This is the most basic unit, representing a single square on the board.

-   **`value: Option<u8>`**: Stores the confirmed digit (1-9) for the cell. `None` if the value is not yet solved.
-   **`is_given: bool`**: `true` if the cell's value is part of the initial puzzle.
-   **`candidates: u16`**: A bitmask representing potential values for the cell. A `1` at the *n*th position means *n* is a possible candidate.

### `Group`
This struct represents a 3x3 block of cells.

-   **`cells: [[Cell; 3]; 3]`**: A 3x3 array containing the `Cell` structs for that block.
-   **`group_values: u16`**: A bitmask tracking which values (1-9) are already present within this 3x3 group.
-   **`group_candidates: [[u16; 3]; 2]`**: Tracks available candidates for each row and column *within* the 3x3 group.

### `Grid`
This is the top-level structure for the entire 9x9 Sudoku board.

-   **`sudoku_cells: [[Group; 3]; 3]`**: A 3x3 array of `Group`s that make up the full grid.
-   **`candidates: [[u16; 9]; 2]`**: Tracks available candidates for each of the 9 global rows and 9 global columns.

## Core Propagation of Values

The solver's logic is centered around a multi-stage propagation mechanism that cascades through the grid whenever a cell's value is set.

1.  **Initial `Grid.set_value()` Call**:
    *   The user or solver calls `grid.set_value(row, col, val, ...)`.
    *   The function identifies the target `Group` (a 3x3 block) and the specific `Cell` within it.
    *   It then delegates the operation by calling `set_value()` on the identified `Group`.

2.  **Intra-Group Propagation (`Group::set_value`)**:
    *   The `Cell`'s value is set, and its `candidates` bitmask is cleared.
    *   The `Group` updates its internal `group_values` bitmask to include the new value.
    *   It then triggers `Group::propagate_changes()`, which:
        *   Eliminates the new value as a candidate from all other unsolved `Cell`s *within the same 3x3 Group*.
        *   Updates the `group_candidates` to reflect that the value is no longer available in the internal rows/columns of that group.

3.  **Inter-Group Propagation (`Grid::propagate_changes`)**:
    *   After the group-level propagation finishes, control returns to the `Grid`.
    *   The `Grid` calls its own `propagate_changes()` method to handle the wider, "flood-fill" effect:
        *   It updates the global `candidates` bitmasks for the entire row and column where the value was set.
        *   It identifies the other two `Group`s in the same "row of groups" and the other two `Group`s in the same "column of groups".
        *   For each of these affected `Group`s, it calls `recive_propagation()`, passing the new value and the relevant row/column index.

4.  **Receiving Propagation (`Group::recive_propagation`)**:
    *   This function is called on a `Group` when a change occurs *outside* of it but in the same row or column.
    *   It receives the value and the local row/column index.
    *   It then eliminates that value as a candidate from all cells in the specified local row or column within its own 3x3 `cells` array.

This systematic, cascading elimination of candidates is the core "human-style" solving technique implemented in the program.