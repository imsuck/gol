use clap::Parser;
use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal::LeaveAlternateScreen;
use gol::tui::start;

use std::io::stdout;

fn main() {
    let Args {
        width,
        height,
        density,
        fps,
    } = Args::parse();

    if let Err(e) = start(width, height, density, fps) {
        crossterm::terminal::disable_raw_mode().ok();
        execute!(stdout(), LeaveAlternateScreen, cursor::Show).ok();
        eprintln!("Error: {e:?} (crossterm)");
    }
}

#[derive(Parser, Debug)]
#[clap(name = "gol", about = "Conway's Game of Life")]
struct Args {
    /// Board width
    #[clap(short = 'w', long = "width", default_value = "20")]
    width: usize,
    /// Board height
    #[clap(short = 'h', long = "height", default_value = "15")]
    height: usize,
    /// Percentage of initial alive cells
    #[clap(short = 'd', long = "density", default_value = "0.3")]
    density: f64,
    #[clap(long = "fps", default_value = "30")]
    fps: u64,
}
