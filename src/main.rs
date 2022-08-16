use std::collections::VecDeque;
use std::io::{stdout, Stdout, Write};
use std::time::{Duration, Instant};
use std::{thread, usize};

use rand::seq::SliceRandom;
use rand::Rng;
use termion::screen::AlternateScreen;
use termion::{clear, color, cursor, terminal_size};

struct MatrixDroplet {
    filled: i32,
    empty: i32,
    pos_x: i32,
    pos_y: i32,
}

impl MatrixDroplet {
    fn new(pos_x: i32) -> MatrixDroplet {
        let mut rng = rand::thread_rng();
        MatrixDroplet {
            filled: rng.gen_range(2..=4),
            empty: rng.gen_range(1..=4),
            pos_x,
            pos_y: -1,
        }
    }

    fn touch_top(&self) -> bool {
        self.filled + self.empty < self.pos_y
    }

    fn should_drop(&self, screen_height: u32) -> bool {
        self.pos_y - screen_height as i32 >= self.filled + self.empty
    }

    fn update(&mut self, buffer: &mut [char], codes: &[char], screen_height: u32) {
        self.pos_y += 1;
        let mut rng = rand::thread_rng();
        let index_top: usize = self.pos_y.try_into().unwrap();
        if index_top < screen_height as usize {
            buffer[index_top] = *codes.choose(&mut rng).unwrap();
        }

        let index_bot = self.pos_y - self.filled;
        if index_bot > 0 && index_bot < screen_height as i32 {
            let index_bot: usize = index_bot.try_into().unwrap();
            buffer[index_bot] = char::from_u32(20).unwrap();
        }
    }

    fn draw(&self, buffer: &[char], height: u32, screen: &mut AlternateScreen<Stdout>) {
        if self.pos_y < 0 {
            return;
        }

        let low: usize = if self.pos_y > self.filled {
            (self.pos_y - self.filled).try_into().unwrap()
        } else {
            0
        };

        let high: usize = if self.pos_y > height as i32 {
            height.try_into().unwrap()
        } else {
            self.pos_y.try_into().unwrap()
        };

        for i in low..high {
            write!(
                screen,
                "{}{}{}",
                buffer[low],
                cursor::Goto(self.pos_x.try_into().unwrap(), (i + 1).try_into().unwrap()),
                color::Fg(color::LightGreen)
            )
            .unwrap();
        }
        write!(
            screen,
            "{}{}{}",
            buffer[high],
            cursor::Goto(
                self.pos_x.try_into().unwrap(),
                (self.pos_y + 1).try_into().unwrap()
            ),
            color::Fg(color::White)
        )
        .unwrap();
    }
}

struct MatrixLine<'a> {
    buffer: Vec<char>,
    droplets: VecDeque<MatrixDroplet>,
    pos_x: i32,
    speed: u32,
    last_updated: Instant,
    screen_height: u32,
    codes: &'a [char],
}

impl<'a> MatrixLine<'a> {
    fn new(screen_height: u32, pos_x: i32, codes: &'a [char]) -> MatrixLine {
        MatrixLine {
            buffer: (1..=screen_height)
                .map(|_| char::from_u32(20).unwrap())
                .collect::<Vec<char>>(),
            droplets: VecDeque::from([MatrixDroplet::new(pos_x)]),
            speed: rand::thread_rng().gen_range(1..=7) * 100_000_000,
            pos_x,
            last_updated: Instant::now(),
            screen_height,
            codes,
        }
    }

    fn get_speed(&self) -> u32 {
        self.speed
    }

    fn update(&mut self) {
        if Instant::now().duration_since(self.last_updated) < Duration::new(0, self.speed) {
            return;
        }

        self.droplets
            .iter_mut()
            .for_each(|d| d.update(&mut self.buffer, self.codes, self.screen_height));

        if self.droplets[0].should_drop(self.screen_height) {
            self.droplets.pop_front();
        }

        if !self.droplets[self.droplets.len() - 1].touch_top() {
            self.droplets.push_back(MatrixDroplet::new(self.pos_x));
        }

        self.last_updated = Instant::now();
    }

    fn draw(&self, screen: &mut AlternateScreen<Stdout>) {
        self.droplets
            .iter()
            .for_each(|d| d.draw(&self.buffer, self.screen_height, screen));
    }
}

impl<'a> std::ops::Index<usize> for MatrixLine<'a> {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

struct Matrix<'a, 'b> {
    screen: &'a mut AlternateScreen<Stdout>,
    lines: Vec<MatrixLine<'b>>,
    update_rate: u32,
}

impl<'a, 'b> Matrix<'a, 'b> {
    fn new(screen: &'a mut AlternateScreen<Stdout>, codes: &'b [char]) -> Matrix<'a, 'b> {
        let size = terminal_size().unwrap();
        let mut update_rate = 700_000_000;
        let mut lines = Vec::new();

        for i in 0..size.0 {
            let line = MatrixLine::new(size.1 as u32, i as i32, codes);
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

    fn update(&mut self) {
        self.lines.iter_mut().for_each(|l| l.update());
    }

    fn draw(&mut self) {
        self.lines.iter_mut().for_each(|l| l.draw(self.screen));
    }

    fn flush(&mut self) {
        self.screen.flush().unwrap();
    }

    fn clear(&mut self) {
        write!(self.screen, "{}", clear::All).unwrap();
    }

    fn sleep(&self) {
        thread::sleep(Duration::new(0, self.update_rate));
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
        let codes = get_matrix_codes();

        let mut matrix = Matrix::new(&mut screen, &codes);

        loop {
            matrix.clear();
            matrix.draw();
            matrix.update();
            matrix.flush();
            matrix.sleep();
        }
    }
}
