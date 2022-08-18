use std::io::{Stdout, Write};

use rand::seq::SliceRandom;
use rand::Rng;
use termion::screen::AlternateScreen;
use termion::{color, cursor};

pub struct MatrixDroplet {
    filled: i32,
    empty: i32,
    pos_x: i32,
    pos_y: i32,
}

impl MatrixDroplet {
    pub fn new(pos_x: i32, screen_height: u32) -> MatrixDroplet {
        let mut rng = rand::thread_rng();
        MatrixDroplet {
            filled: rng.gen_range((screen_height as i32 / 2)..=(screen_height as i32)),
            empty: rng.gen_range((screen_height as i32 / 3)..=(screen_height as i32 / 2)),
            pos_x,
            pos_y: -rng.gen_range(0..=(screen_height as i32 / 2)),
        }
    }

    pub fn touch_top(&self) -> bool {
        self.filled + self.empty <= self.pos_y
    }

    pub fn should_drop(&self, screen_height: u32) -> bool {
        self.pos_y - screen_height as i32 > self.filled + self.empty
    }

    pub fn update(&mut self, buffer: &mut [char], codes: &[char], screen_height: u32) {
        self.pos_y += 1;
        if self.pos_y < 0 {
            return;
        }

        let mut rng = rand::thread_rng();
        let index_top: usize = self.pos_y.try_into().unwrap();
        if index_top < screen_height as usize {
            buffer[0] = buffer[1];
            buffer[1] = *codes.choose(&mut rng).unwrap();
        }
    }

    pub fn draw(&self, buffer: &[char], height: u32, screen: &mut AlternateScreen<Stdout>) {
        if self.pos_y < 0 {
            return;
        }

        if self.pos_y > self.filled {
            let low = (self.pos_y - self.filled) as u16;
            write!(screen, "{}  ", cursor::Goto(self.pos_x as u16, low),).unwrap();
        }

        if self.pos_y > 0 && self.pos_y <= height as i32 {
            write!(
                screen,
                "{}{}{} ",
                cursor::Goto(self.pos_x as u16, self.pos_y as u16),
                color::Fg(color::Green),
                buffer[0],
            )
            .unwrap();
        }

        if self.pos_y < height as i32 {
            write!(
                screen,
                "{}{}{} ",
                cursor::Goto(self.pos_x as u16, (self.pos_y + 1) as u16),
                color::Fg(color::White),
                buffer[1]
            )
            .unwrap();
        }
    }
}
