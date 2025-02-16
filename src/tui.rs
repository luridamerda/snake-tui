use crate::util::Vec2;

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

pub struct Window {
    pos: Vec2,
    size: Vec2,
    title: Option<String>
}

impl Window {

    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            pos: Vec2{x, y},
            size: Vec2{x: width, y: height},
            title: None,
        }
    }

    pub fn centered(renderer: &Renderer, width: u16, height: u16) -> Self {
        let (x, y) = renderer.center_point();

        Self {
            pos: Vec2{x: x - width/2, y: y - height/2},
            size: Vec2{x: width, y: height},
            title: None
        }
            
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    pub fn inner(&self) -> Self {
        Self {
            pos: Vec2{x: self.pos.x+1, y: self.pos.y+1},
            size: Vec2{x: self.size.x-2, y: self.size.y-2},
            title: None,
        }
    }

    pub fn outer(&self) -> Self {
        Self {
            pos: Vec2{x: self.pos.x-1, y: self.pos.y-1},
            size: Vec2{x: self.size.x+2, y: self.size.y+2},
            title: None,
        }
    }

    pub fn pixel(&self, renderer: &mut Renderer, x: u16, y: u16, c: char) -> Result<(), io::Error> {
        renderer.pixel(self.pos.x + x, self.pos.y + y, c)?;
        Ok(())
    }

    pub fn pixel_styled(&self, renderer:&mut Renderer, x: u16, y: u16, c: StyledContent<char>) -> Result<(), io::Error> {
        renderer.pixel_styled(self.pos.x + x, self.pos.y + y, c)?;
        Ok(())
    }

    pub fn print_str(&self, renderer: &mut Renderer, x: u16, y: u16, s: &str) -> Result<(), io::Error> {
        renderer.print_str(x + self.pos.x, y + self.pos.y, s)?;
        Ok(())
    } 

    pub fn print_centered_str(&self, renderer: &mut Renderer, y: u16, s: &str) -> Result<(), io::Error> {
        renderer.print_str(self.size.x / 2 - (s.chars().count() / 2) as u16 + self.pos.x, y + self.pos.y, s)?;
        Ok(())
    } 

    pub fn draw_borders(&self, renderer: &mut Renderer) -> Result<(), io::Error> {
        for y in 1..self.size.y {
            self.pixel(renderer, 0, y,'│')?;
            self.pixel(renderer, self.size.x, y,'│')?;
        }
        for x in 1..self.size.x {
            self.pixel(renderer, x, 0,'─')?;
            self.pixel(renderer, x, self.size.y,'─')?;
        }

        self.pixel(renderer, 0, 0,'┌')?;
        self.pixel(renderer, self.size.x, 0,'┐')?;
        self.pixel(renderer, 0, self.size.y,'└')?;
        self.pixel(renderer, self.size.x, self.size.y,'┘')?;

        if let Some(name) = &self.title {
            let title = format!("[ {} ]", name);
            self.print_centered_str(renderer, 0, &title)?;
        }

        Ok(())
    }
}

pub struct Renderer {
    stdout: Stdout,
    game_window: Window,
}

impl Renderer {
    pub fn new() -> Self {
        let (columns, rows) = size().unwrap();

        Self {
            stdout: io::stdout(),
            game_window: Window::new(columns / 2 - 34 / 2 , rows / 2 - 26 / 2 - 1, 34, 26),
        }
    }

    pub fn center_point(&self) -> (u16, u16) {
        let (x, y) = size().unwrap();
        ((x/2) as u16, (y/2) as u16)
    }

    pub fn init(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        self.stdout.execute(cursor::Hide)?;

        Ok(())
    }

    pub fn dispose(&mut self) -> Result<(), io::Error> {
        self.stdout.execute(cursor::Show)?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), io::Error> {
        queue!(self.stdout, Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn present(&mut self) -> Result<(), io::Error> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn pixel(&mut self, x: u16, y: u16, c: char) -> Result<(), io::Error> {
        self.pixel_styled(x, y, StyledContent::new(ContentStyle::new(), c))?;
        Ok(())
    }

    pub fn pixel_styled(&mut self, x: u16, y: u16, c: StyledContent<char>) -> Result<(), io::Error> {
        queue!(self.stdout, MoveTo(x, y), Print(&c))?;
        Ok(())
    }

    pub fn print_str(&mut self, x: u16, y: u16, s: &str) -> Result<(), io::Error> {
        queue!(self.stdout, MoveTo(x, y), Print(s))?;
        Ok(())
    } 
}
