use crossterm::{
    cursor,
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    queue,
    style::{Print, StyledContent, Stylize, Color, ContentStyle},
    terminal::{self, size, disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Stdout, Write};
use std::{thread, time::Duration};

mod game;
mod util;
mod tui;

use crate::game::{Game, Direction, Tile, GameState, FIELD_LINES, FIELD_COLS};
use crate::util::Vec2;
use crate::tui::{Window, Renderer};

fn draw_tile(renderer: &mut Renderer, window: &Window, x: u16, y: u16, t: &Tile) -> Result<(), io::Error> {
    let tile_ch = match t {
        Tile::Snake(v) => ' '.on_green(),
        Tile::Apple => ' '.on_red(),
        _ => ' '.blue(),
    };

    window.inner().pixel_styled(renderer, x*2, y, tile_ch)?;
    window.inner().pixel_styled(renderer, x*2+1, y, tile_ch)?;

    Ok(())
}

fn draw_game(renderer: &mut Renderer, window: &mut Window, game: &Game) -> Result<(), io::Error> {

    let title = format!("Apples: {}", game.points());
    window.set_title(&title);
    window.draw_borders(renderer)?;

    for y in 0..game.field().len() {
        for x in 0..game.field()[0].len() {
            draw_tile(renderer, window, x as u16, y as u16, &game.field()[y][x])?;
        }
    }

    Ok(())
}

fn draw_main_menu(renderer: &mut Renderer, window: &mut Window) -> Result<(), io::Error> {
    window.set_title("Snake");
    window.draw_borders(renderer)?;

    window.print_centered_str(renderer, 2, "Snake game in the terminal")?;
    window.print_centered_str(renderer, 3, "written in Rust")?;
    window.print_centered_str(renderer, 5, "Use arrow keys ← → ↑ ↓ to move")?;
    window.print_centered_str(renderer, 7, "Press ESC to exit")?;
    Ok(())
}

fn draw_end_menu(renderer: &mut Renderer, window: &mut Window, points: u16) -> Result<(), io::Error> {
    window.set_title("Game Over");
    window.draw_borders(renderer)?;


    let p = format!("You ate {} apples", points);
    window.print_centered_str(renderer, 2, &p)?;
    window.print_centered_str(renderer, 4, "Use arrow keys ← → ↑ ↓ to restart")?;

    Ok(())
}

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new();
    let mut game = Game::new();

    let mut win = Window::centered(&renderer, (FIELD_COLS * 2 + 2) as u16, (FIELD_LINES + 2) as u16);

    renderer.init()?;

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            if let Ok(event) = read() {
                if let Event::Key(key) = event {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Up => game.move_to(Direction::Up),
                        KeyCode::Down => game.move_to(Direction::Down),
                        KeyCode::Left => game.move_to(Direction::Left),
                        KeyCode::Right => game.move_to(Direction::Right),
                        _ => {}
                    }
                }
            }
        }

        game.step();

        match game.state() {
            GameState::Starting => draw_main_menu(&mut renderer, &mut win)?,
            GameState::Started => draw_game(&mut renderer, &mut win, &game)?,
            GameState::Ended => {
                renderer.clear()?;
                draw_end_menu(&mut renderer, &mut win, game.points())?;
            }
        }


        renderer.present()?;
    }

    renderer.dispose()?;

    Ok(())
}
