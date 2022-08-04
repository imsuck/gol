//! Tui for `GoL`

use crate::Game;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use std::io::stdout;
use std::thread;
use std::time::Duration;

enum Action {
    NextGen,
}

/// Start `GoL` tui
pub fn start(width: usize, height: usize, density: f64, fps: u64) -> crossterm::Result<()> {
    let _clean_up = CleanUp;

    execute!(stdout(), EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    let mut game = Game::new_rand(width, height, density);

    loop {
        execute!(stdout(), Clear(ClearType::All)).ok();
        print!("{game}");
        game.update_board();
        if event::poll(Duration::from_secs(0)).ok().unwrap_or(false) {
            if let Some(Event::Key(key)) = event::read().ok() {
                match (key.modifiers, key.code) {
                    // Restart
                    (_, KeyCode::Char('r')) => game = Game::new_rand(width, height, density),
                    // Pause
                    (_, KeyCode::Char(' ')) => loop {
                        if let Some(Event::Key(_)) = event::read().ok() {
                            break;
                        }
                    },
                    (KeyModifiers::CONTROL, KeyCode::Char('c'))
                    | (_, KeyCode::Char('q') | KeyCode::Esc) => break,
                    _ => (),
                }
            }
        }
        thread::sleep(Duration::from_millis(1000 / fps));
    }

    Ok(())
}

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().ok();
        execute!(stdout(), LeaveAlternateScreen).ok();
    }
}
