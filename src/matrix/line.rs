use std::collections::VecDeque;
use std::io::Stdout;
use std::time::{Duration, Instant};

use rand::Rng;
use termion::screen::AlternateScreen;

use crate::matrix::droplet::MatrixDroplet;

pub struct MatrixLine<'a> {
    buffer: [char; 2],
    droplets: VecDeque<MatrixDroplet>,
    pos_x: i32,
    speed: u32,
    last_updated: Instant,
    screen_height: u32,
    codes: &'a [char],
    should_draw: bool,
}

impl<'a> MatrixLine<'a> {
    pub fn new(screen_height: u32, pos_x: i32, codes: &'a [char]) -> MatrixLine {
        let space = char::from_u32(20).unwrap();
        MatrixLine {
            buffer: [space, space],
            droplets: VecDeque::from([MatrixDroplet::new(pos_x, screen_height)]),
            speed: rand::thread_rng().gen_range(38..=60) * 1_000_000,
            pos_x,
            last_updated: Instant::now(),
            screen_height,
            codes,
            should_draw: false,
        }
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn update(&mut self) {
        if Instant::now().duration_since(self.last_updated) < Duration::new(0, self.speed) {
            return;
        }

        self.droplets
            .iter_mut()
            .for_each(|d| d.update(&mut self.buffer, self.codes, self.screen_height));

        if self.droplets[0].should_drop(self.screen_height) {
            self.droplets.pop_front();
        }

        if self.droplets[self.droplets.len() - 1].touch_top() {
            self.droplets
                .push_back(MatrixDroplet::new(self.pos_x, self.screen_height));
        }

        self.should_draw = true;
        self.last_updated = Instant::now();
    }

    pub fn draw(&mut self, screen: &mut AlternateScreen<Stdout>) {
        if !self.should_draw {
            return;
        }

        self.droplets
            .iter()
            .for_each(|d| d.draw(&self.buffer, self.screen_height, screen));

        self.should_draw = false;
    }
}

impl<'a> std::ops::Index<usize> for MatrixLine<'a> {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}
