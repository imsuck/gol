//! Game elements

use rand::{thread_rng, Rng};
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
        for (i, row) in self.board.iter().enumerate() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\r")?;
            if i != self.board.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Game {
    /// Creates a new game with random board
    #[must_use]
    pub fn new_rand(width: usize, height: usize, density: f64) -> Self {
        let mut game = Self::new_blank(width, height);

        for row in &mut game.board {
            for cell in row.iter_mut() {
                *cell = if thread_rng().gen_bool(density) {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }
        }

        game
    }
    /// Creates a new game filled with Dead cells
    #[must_use]
    pub fn new_blank(width: usize, height: usize) -> Self {
        Self {
            board: vec![vec![Cell::Dead; width]; height],
        }
    }

    /// Updates the board according to the following rules:
    ///
    /// - Any alive cell with fewer than 2 or more than 3 alive neighbours dies.
    ///
    /// - Any dead cells with exactly 3 alive neighbours comes to life.
    pub fn update_board(&mut self) {
        let current = self.clone();
        let mut next = self.clone();

        for (x, row) in next.board.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                let neighbours = vec![
                    current.cell((subtract_one(x), subtract_one(y))),
                    current.cell((x, subtract_one(y))),
                    current.cell((x.saturating_add(1), subtract_one(y))),
                    current.cell((subtract_one(x), y)),
                    current.cell((x.saturating_add(1), y)),
                    current.cell((subtract_one(x), y.saturating_add(1))),
                    current.cell((x, y.saturating_add(1))),
                    current.cell((x.saturating_add(1), y.saturating_add(1))),
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

        self.board = next.board;
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

// Helper function to help with checking neighbours of border cells
fn subtract_one(x: usize) -> usize {
    x.checked_sub(1).unwrap_or(usize::MAX)
}
