//! Library for Conway's Game of Life

#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    rust_2018_idioms,
    missing_docs
)]

use std::fmt;

/// The position
type Pos = (usize, usize);

/// Cell state
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alive => write!(f, "⬜"),
            Self::Dead => write!(f, "⬛"),
        }
    }
}

/// The game which currently is just a board
#[derive(Clone, Debug)]
pub struct Game {
    /// The board which contains every cells in the game
    // Row<Column<Cell>>
    board: Vec<Vec<Cell>>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Game {
    /// Creates a new game filled with Dead cells
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            board: vec![vec![Cell::Dead; size]; size],
        }
    }

    /// Updates the board according to the following rules:
    ///
    /// - Any alive cell with fewer than 2 or more than 3 alive neighbours dies.
    ///
    /// - Any dead cells with exactly 3 alive neighbours comes to life.
    pub fn update_board(&mut self) {
        let old_board = self.clone();
        let mut new_board = self.clone();

        for (x, row) in new_board.board.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                let neighbours = vec![
                    old_board.cell((subtract_one(x), subtract_one(y))),
                    old_board.cell((x, subtract_one(y))),
                    old_board.cell((x.saturating_add(1), subtract_one(y))),
                    old_board.cell((subtract_one(x), y)),
                    old_board.cell((x.saturating_add(1), y)),
                    old_board.cell((subtract_one(x), y.saturating_add(1))),
                    old_board.cell((x, y.saturating_add(1))),
                    old_board.cell((x.saturating_add(1), y.saturating_add(1))),
                ];

                let alive_neighbours_count: usize = neighbours
                    .iter()
                    .filter(|neighbour| **neighbour == Some(&Cell::Alive))
                    .count();

                *cell = match alive_neighbours_count {
                    0..=1 | 4.. => Cell::Dead,
                    2 => *cell,
                    3 => Cell::Alive,
                    _ => unreachable!(),
                }
            }
        }

        self.board = new_board.board;
    }

    /// *Debug only*
    ///
    /// Flips the cell in the specified position
    /// # Panics
    /// This method will panic when the position specified is not in the board
    #[cfg(debug_assertions)]
    pub fn flip(&mut self, pos: Pos) {
        self.board[pos.1][pos.0] = match self.cell(pos).unwrap() {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }

    /// Gets a reference to a cell
    fn cell(&self, pos: Pos) -> Option<&Cell> {
        self.board.get(pos.0)?.get(pos.1)
    }
}

// Utility function to help with checking neighbours of border cells
fn subtract_one(x: usize) -> usize {
    x.checked_sub(1).unwrap_or(usize::MAX)
}
