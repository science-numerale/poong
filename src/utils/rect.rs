use crossterm::{
    QueueableCommand, cursor,
    style::{Color, Print, SetBackgroundColor},
};
use std::io::Write;

use crate::{BACKGROUND_COLOR, STD_OUT};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: Color,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize, color: Color) -> Self {
        let slf = Self { x, y, w, h, color };
        slf.show();
        slf
    }

    pub fn show(&self) {
        let mut stdout = STD_OUT.lock().unwrap();

        stdout.queue(SetBackgroundColor(self.color)).unwrap();

        for x in 0..self.w {
            for y in 0..self.h {
                stdout
                    .queue(cursor::MoveTo((self.x + x) as u16, (self.y + y) as u16))
                    .unwrap();
                stdout.queue(Print(" ")).unwrap();
            }
        }

        stdout.flush().unwrap();
    }

    pub fn move_to(&mut self, new_x: usize, new_y: usize) {
        let mut stdout = STD_OUT.lock().unwrap();
        let old = *self;
        self.x = new_x;
        self.y = new_y;

        let bg = *BACKGROUND_COLOR.lock().unwrap();

        for x in 0..self.w {
            for y in 0..self.h {
                if !self.contains(old.x + x, old.y + y) {
                    stdout.queue(SetBackgroundColor(bg)).unwrap();
                    stdout
                        .queue(cursor::MoveTo((old.x + x) as u16, (old.y + y) as u16))
                        .unwrap();
                    stdout.queue(Print(" ")).unwrap();
                }

                if !old.contains(self.x + x, self.y + y) {
                    stdout.queue(SetBackgroundColor(self.color)).unwrap();
                    stdout
                        .queue(cursor::MoveTo((self.x + x) as u16, (self.y + y) as u16))
                        .unwrap();
                    stdout.queue(Print(" ")).unwrap();
                }
            }
        }

        stdout.flush().unwrap();
    }

    pub const fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }

    pub const fn x(&self) -> usize {
        self.x
    }

    pub const fn y(&self) -> usize {
        self.y
    }
}
