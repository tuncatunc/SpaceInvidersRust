use std::time::Duration;

use rusty_time::prelude::Timer;

use crate::frame::{self, Drawable};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub is_alive: bool,
    pub timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            is_alive: true,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(&mut self, duration: Duration) {
        self.timer.update(duration);

        if self.timer.ready && self.is_alive {
            if self.y > 0 {
                self.y -= 1;
            } else {
                self.is_alive = false;
            }
            self.timer.reset()
        }
    }

    pub fn explode(&mut self) {
        self.is_alive = false;
        self.timer = Timer::from_millis(200);
    }

    pub fn is_exploded(&self) -> bool {
        !self.is_alive && self.timer.ready
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut frame::Frame) {
        if !self.is_alive {
            return;
        }
        frame[self.x][self.y] = if self.is_alive { "|" } else { "*" };
    }
}
