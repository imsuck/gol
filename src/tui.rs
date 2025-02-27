//! Tui for `GoL`

use crate::Game;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

use std::io::stdout;
use std::thread;
use std::time::Duration;

enum Action {
    None,
    Quit,
    Pause,
    Restart,
}

/// Start `GoL` tui
pub fn run(width: u32, height: u32, density: f64, fps: u32) -> crossterm::Result<()> {
    let _clean_up = CleanUp;

    execute!(stdout(), EnterAlternateScreen, Hide)?;
    crossterm::terminal::enable_raw_mode()?;
    let mut game = Game::new(width, height, density);

    draw(&game);
    thread::sleep(Duration::from_micros(1_000_000 / fps as u64));

    'out: loop {
        game.tick();
        draw(&game);

        match handle_keypresses() {
            Action::None => (),
            Action::Restart => {
                game = Game::new(width, height, density);
                draw(&game);
            }
            Action::Pause => loop {
                if let Ok(Event::Key(key)) = event::read() {
                    match key.code {
                        KeyCode::Char('n') => {
                            game.tick();
                            draw(&game);
                        }
                        KeyCode::Char('q') => break 'out,
                        _ => break,
                    }
                }
            },
            Action::Quit => break,
        }

        thread::sleep(Duration::from_micros(1_000_000 / fps as u64));
    }

    Ok(())
}

fn handle_keypresses() -> Action {
    if event::poll(Duration::from_secs(0)).ok().unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            match (key.modifiers, key.code) {
                // Restart
                (_, KeyCode::Char('r')) => Action::Restart,
                // Pause
                (_, KeyCode::Char(' ')) => Action::Pause,
                // Quit
                (KeyModifiers::CONTROL, KeyCode::Char('c'))
                | (_, KeyCode::Char('q') | KeyCode::Esc) => Action::Quit,
                _ => Action::None,
            }
        } else {
            Action::None
        }
    } else {
        Action::None
    }
}

fn draw(game: &Game) {
    execute!(stdout(), MoveTo(0, 0)).ok();
    print!("{game}");
}

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().ok();
        execute!(stdout(), Show, LeaveAlternateScreen).ok();
    }
}
