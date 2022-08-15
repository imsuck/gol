//! Game logic

use fixedbitset::FixedBitSet;
use rand::{thread_rng, Rng};

use std::fmt;

/// The game
#[derive(Clone, Debug)]
pub struct Game {
    // Row<Column<Cell>>
    board: FixedBitSet,
    width: u32,
    height: u32,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);

                let symbol = if self.board.contains(index) {
                    '⬜'
                } else {
                    '⬛'
                };

                write!(f, "{}", symbol)?;
            }

            write!(f, "\r")?;
            if row != self.height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Game {
    /// Creates a new game with random board
    #[must_use]
    pub fn new(width: u32, height: u32, density: f64) -> Self {
        let size = (width * height) as usize;

        let mut board = FixedBitSet::with_capacity(size);

        for i in 0..size {
            board.set(i, thread_rng().gen_bool(density));
        }

        Self {
            board,
            width,
            height,
        }
    }

    /// Updates the board according to the following rules:
    ///
    /// - Any live cell with fewer than 2 or more than 3 live neighbours dies.
    ///
    /// - Any dead cells with exactly 3 live neighbours comes to life.
    pub fn tick(&mut self) {
        let mut next = self.board.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.board.contains(index);
                let live_neighbour_count = self.live_neighbour_count(row, col);

                next.set(
                    index,
                    match (cell, live_neighbour_count) {
                        (_, x) if !(2..=3).contains(&x) => false,
                        (_, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.board = next;
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }

                let neighbour_col = (column + delta_col) % self.width;
                let neighbour_row = (row + delta_row) % self.height;

                let index = self.get_index(neighbour_row, neighbour_col);

                count += self.board.contains(index) as u8;
            }
        }

        count
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}
