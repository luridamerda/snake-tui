use crossterm::{
    cursor,
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    queue,
    style::{Print, StyledContent, Stylize, Color},
    terminal::{self, size, disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::io::{self, stdout, Stdout, Write};
use std::{thread, time::Duration};

const FIELD_LINES: usize = 20;
const FIELD_COLS: usize = 30;

#[derive(Debug, Copy, Clone)]
enum Tile {
    Snake(u16),
    Apple,
    Free,
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Snake(_) => ' ',
            Tile::Apple => ' ',
            Tile::Free => ' ',
        }
    }
}

type Field = [[Tile; FIELD_COLS]; FIELD_LINES];

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(PartialEq)]
enum GameState {
    Starting,
    Started,
    Ended,
}

#[derive(Debug)]
struct Vec2 {
    x: u16,
    y: u16,
}

impl Vec2 {
    fn new(x: u16, y: u16) -> Self {
        Self{x, y}
    }
}

struct Game {
    head: Vec2,
    length: u16,
    direction: Direction,
    field: Field,
    state: GameState,
}

impl Game {
    fn new() -> Self {
        let mut field = [[Tile::Free; FIELD_COLS]; FIELD_LINES];
        field[0][0] = Tile::Snake(1);
        field[10][10] = Tile::Apple;

        Self {
            head: Vec2::new(0, 0),
            length: 1,
            direction: Direction::Right,
            field,
            state: GameState::Started,
        }
    }

    fn points(&self) -> u16 {
        self.length - 1
    }

    fn field(&self) -> &Field {
        &self.field
    }

    fn move_to(&mut self, d: Direction) {
        if self.direction.opposite() != d {
            self.direction = d;
        }
    }

    fn next_frame(&mut self) {
        if let GameState::Started = self.state {
            for row in self.field.iter_mut() {
                for elem in row.iter_mut() {
                    *elem = update_element(*elem);
                }
            }

            self.move_head();

            let ended = self.handle_collisions();

            if !ended {
                self.set_head(Tile::Snake(self.length));
            }
        }
    }

    fn head(&self) -> Tile {
        self.field[self.head.y as usize][self.head.x as usize]
    }

    fn set_head(&mut self, e: Tile) {
        self.field[self.head.y as usize][self.head.x as usize] = e;
    }

    fn handle_collisions(&mut self) -> bool {

        match self.head() {
            Tile::Snake(_) => self.state = GameState::Ended,
            Tile::Apple => {
                self.length += 1;
                self.create_apple();
            },
            _ => {}
        };

        self.state == GameState::Ended
    }

    fn move_head(&mut self) {

        self.head.x = (self.head.x as i32 + match self.direction {
            Direction::Left => -1i32,
            Direction::Right => 1i32,
            _ => 0,
        }).rem_euclid(self.field[0].len() as i32) as u16;

        self.head.y = (self.head.y as i32 + match self.direction {
            Direction::Up => -1i32,
            Direction::Down => 1i32,
            _ => 0,
        }).rem_euclid(self.field.len() as i32) as u16;
    }

    fn create_apple(&mut self) {
        let mut rng = rand::thread_rng();
        let mut gen = false;

        while !gen {
            let line: usize = rng.gen_range(0..FIELD_LINES);
            let col: usize = rng.gen_range(0..FIELD_COLS);

            if let Tile::Free = self.field[line][col] {
                gen = true;
                self.field[line][col] = Tile::Apple;
            }

        }


    }
}

fn update_element(e: Tile) -> Tile {
    match e {
        Tile::Snake(index) => {
            if index == 1 {
                Tile::Free
            } else {
                Tile::Snake(index - 1)
            }
        }
        _ => e,
    }
}

struct Window {
    pos: Vec2,
    size: Vec2,
    title: Option<String>
}

impl Window {

    fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            pos: Vec2{x, y},
            size: Vec2{x: width, y: height},
            title: None,
        }
    }

    fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    fn inner(&self) -> Self {
        Self {
            pos: Vec2{x: self.pos.x+1, y: self.pos.y+1},
            size: Vec2{x: self.size.x-2, y: self.size.y-2},
            title: None,
        }
    }

    fn outer(&self) -> Self {
        Self {
            pos: Vec2{x: self.pos.x-1, y: self.pos.y-1},
            size: Vec2{x: self.size.x+2, y: self.size.y+2},
            title: None,
        }
    }

    fn pixel_pos(&self, stdout: &mut Stdout, p: &Vec2, c: char) -> Result<(), io::Error> {
        self.pixel(stdout, p.x, p.y, c)?;
        Ok(())
    }

    fn pixel(&self, stdout: &mut Stdout, x: u16, y: u16, c: char) -> Result<(), io::Error> {
        queue!(stdout, MoveTo(x + self.pos.x, y + self.pos.y), Print(&c))?;
        Ok(())
    }

    fn pixel_styled(&self, stdout: &mut Stdout, x: u16, y: u16, c: StyledContent<char>) -> Result<(), io::Error> {
        queue!(stdout, MoveTo(x + self.pos.x, y + self.pos.y), Print(&c))?;
        Ok(())
    }

    fn print_str(&self, stdout: &mut Stdout, x: u16, y: u16, s: &str) -> Result<(), io::Error> {
        queue!(stdout, MoveTo(x + self.pos.x, y + self.pos.y), Print(s))?;
        Ok(())
    } 

    fn draw(&self, stdout: &mut Stdout) -> Result<(), io::Error> {
        for y in 1..self.size.y {
            self.pixel(stdout, 0, y,'│')?;
            self.pixel(stdout, self.size.x, y,'│')?;
        }
        for x in 1..self.size.x {
            self.pixel(stdout, x, 0,'─')?;
            self.pixel(stdout, x, self.size.y,'─')?;
        }

        self.pixel(stdout, 0, 0,'┌')?;
        self.pixel(stdout, self.size.x, 0,'┐')?;
        self.pixel(stdout, 0, self.size.y,'└')?;
        self.pixel(stdout, self.size.x, self.size.y,'┘')?;

        if let Some(name) = &self.title {
            let title = format!("┤ {} ├", name);
            self.print_str(stdout, self.size.x / 2 - name.len() as u16 / 2, 0, &title)?;
        }

        Ok(())
    }
}

struct Renderer {
    stdout: Stdout,
    game_window: Window
}

impl Renderer {
    fn new() -> Self {

        let (columns, rows) = size().unwrap();
        let width = (FIELD_COLS*2 + 2) as u16;
        let height = (FIELD_LINES + 1) as u16;

        Self {
            stdout: io::stdout(),
            game_window: Window::new(columns / 2 - width / 2 , rows / 2 - height / 2, width, height)
        }
    }

    fn init(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        self.stdout.execute(cursor::Hide)?;

        Ok(())
    }

    fn dispose(&mut self) -> Result<(), io::Error> {
        self.stdout.execute(cursor::Show)?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    fn clear(&mut self) -> Result<(), io::Error> {
        queue!(self.stdout, Clear(terminal::ClearType::All))?;
        Ok(())
    }

    fn present(&mut self) -> Result<(), io::Error> {
        self.stdout.flush()?;
        Ok(())
    }

    fn pixel(&mut self, p: Vec2, c: StyledContent<char>) -> Result<(), io::Error> {
        queue!(self.stdout, MoveTo(p.x, p.y), Print(&c))?;
        Ok(())
    }

    fn draw_tile(&mut self, p: Vec2, t: &Tile) -> Result<(), io::Error> {

        let tile_ch = match t {
            Tile::Snake(v) => ' '.on_green(),
            Tile::Apple => ' '.on_red(),
            _ => ' '.blue(),
        };

        self.game_window.inner().pixel_styled(&mut self.stdout, p.x, p.y, tile_ch)?;

        Ok(())
    }

    fn render(&mut self, game: &Game) -> Result<(), io::Error> {
        let size = Vec2 {
            x: ((game.field()[0].len() + 1) * 2) as u16,
            y: (game.field().len() + 1) as u16,
        };

        let title = format!("Apples: {}", game.points());
        self.game_window.set_title(&title);
        self.game_window.draw(&mut self.stdout)?;

        for y in 0..game.field().len() {
            for x in 0..game.field()[0].len() {
                self.draw_tile(
                    Vec2 {
                        x: (x * 2) as u16,
                        y: y as u16,
                    },
                    &game.field()[y][x],
                )?;
                self.draw_tile(
                    Vec2 {
                        x: (x * 2 + 1) as u16,
                        y: y as u16,
                    },
                    &game.field()[y][x],
                )?;
            }
        }

        self.present()?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new();

    let mut game = Game::new();

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

        game.next_frame();
        renderer.render(&game)?;

        renderer.present()?;
    }

    renderer.dispose()?;

    Ok(())
}
