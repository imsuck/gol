//! Window handling

use piston_window::{
    rectangle,
    rectangle::{margin, square},
    PistonWindow, WindowSettings,
};

const CELL_SIZE: f64 = 10.0;

/// Run
pub fn run(width: u32, height: u32, density: f64, _fps: u32) {
    let mut game = crate::Game::new(width, height, density);
    let mut window: PistonWindow = WindowSettings::new("Game of Life", (640, 480))
        .exit_on_esc(true)
        .automatic_close(true)
        .build()
        .expect("Failed to create window");

    while let Some(e) = window.next() {
        let content = game.to_string();

        window.draw_2d(&e, |context, graphics, _| {
            for (y, line) in content.lines().enumerate() {
                for (x, cell) in line.chars().enumerate() {
                    let x = x as f64;
                    let y = y as f64;
                    match cell {
                        '⬜' => rectangle(
                            [1.0, 1.0, 1.0, 1.0],
                            margin(
                                square(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE),
                                CELL_SIZE / 15.0,
                            ),
                            context.transform,
                            graphics,
                        ),
                        '⬛' => rectangle(
                            [0.0, 0.0, 0.0, 1.0],
                            square(x * CELL_SIZE as f64, y * CELL_SIZE as f64, CELL_SIZE),
                            context.transform,
                            graphics,
                        ),
                        _ => (),
                    }
                }
            }
        });

        game.tick();
    }
}
