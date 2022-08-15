use std::io::{stdout, Write};
use std::{thread, time, usize};

use rand::Rng;
use termion::screen::AlternateScreen;
use termion::{color, cursor};

struct MatrixLine {
    buffer: Vec<char>,
    speed: u32,
}

impl MatrixLine {
    fn new(height: u32) -> MatrixLine {
        MatrixLine {
            buffer: (0..height)
                .map(|_| char::from_u32(20).unwrap())
                .collect::<Vec<char>>(),
            speed: rand::thread_rng().gen_range(2..=7) * 100,
        }
    }

    fn update(&mut self) {
        self.buffer.pop();
        let prev = self[0];
    }
}

impl std::ops::Index<usize> for MatrixLine {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

fn get_matrix_codes() -> Vec<char> {
    let mut codes = (0x21..0x7E)
        .map(|c| char::from_u32(c).unwrap())
        .collect::<Vec<char>>();
    codes.extend((0xFF66..0xFF9D).map(|c| char::from_u32(c).unwrap()));
    codes
}

fn main() {
    {
        let mut screen = AlternateScreen::from(stdout());

        for r in 0..255 {
            let c = color::Rgb(r, !r, 2 * ((r % 128) as i8 - 64).abs() as u8);
            write!(screen, "{}{}wow", cursor::Goto(1, 1), color::Bg(c),).unwrap();
            screen.flush().unwrap();
            thread::sleep(time::Duration::from_millis(3));
        }

        // write!(screen, "dupaaa").unwrap();
        // screen.flush().unwrap();
        // thread::sleep(time::Duration::from_secs(2));
    }
    println!("Writing to main screen.");
}
