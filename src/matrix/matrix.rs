use std::io::{Stdout, Write};
use std::thread;
use std::time::Duration;

use termion::screen::AlternateScreen;
use termion::{cursor, terminal_size};

use crate::matrix::line::MatrixLine;

pub struct Matrix<'a, 'b> {
    screen: &'a mut AlternateScreen<Stdout>,
    lines: Vec<MatrixLine<'b>>,
    update_rate: u32,
}

impl<'a, 'b> Matrix<'a, 'b> {
    pub fn new(screen: &'a mut AlternateScreen<Stdout>, codes: &'b [char]) -> Matrix<'a, 'b> {
        let size = terminal_size().unwrap();
        let mut update_rate = 700_000_000;
        let mut lines = Vec::new();

        for i in (0..size.0 - 1).step_by(2) {
            let line = MatrixLine::new(size.1 as u32, (i + 1) as i32, codes);
            update_rate = if line.get_speed() < update_rate {
                line.get_speed()
            } else {
                update_rate
            };
            lines.push(line);
        }

        Matrix {
            screen,
            lines,
            update_rate,
        }
    }

    pub fn update(&mut self) {
        self.lines.iter_mut().for_each(|l| l.update());
    }

    pub fn draw(&mut self) {
        self.lines.iter_mut().for_each(|l| l.draw(self.screen));
    }

    pub fn flush(&mut self) {
        self.screen.flush().unwrap();
    }

    pub fn hide_cursor(&mut self) {
        write!(self.screen, "{}", cursor::Hide).unwrap();
    }

    pub fn unhide_cursor(&mut self) {
        write!(self.screen, "{}", cursor::Show).unwrap();
    }

    pub fn sleep(&self) {
        thread::sleep(Duration::new(0, self.update_rate));
    }
}
