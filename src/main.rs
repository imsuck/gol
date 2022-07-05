use std::fmt;

#[derive(Clone, Copy)]
enum Cell {
    Alive,
    Dead,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Alive => write!(f, "⬛"),
            Self::Dead => write!(f, "⬜"),
        }
    }
}

struct Position(usize, usize);

struct Game {
    board: Vec<Vec<Cell>>,
}

impl Game {
    fn new(size: usize) -> Self {
        Self {
            board: vec![vec![Cell::Dead; size]; size],
        }
    }

    fn cell(&self, pos: Position) -> Cell {
        self.board[pos.0][pos.1]
    }
}

fn main() {
    let size = 5;
    let game = Game::new(size);
    for x in 0..size {
        for y in 0..size {
            print!("{}", game.cell(Position(x, y)));
        }
        println!();
    }
}
