use crossterm::{
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
    cursor::{
        MoveTo
    }
};
use std::io::{Stdout, Write};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(SetForegroundColor(Color::White)).unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, cell) in col.iter().enumerate() {
            if last_frame[x][y] != *cell {
                stdout
                    .queue(MoveTo(x as u16, y as u16))
                    .unwrap();
                print!("{}", cell);
            }
        }
    }
    stdout.flush().unwrap();
}