use crossterm::{
    cursor, style::Color, terminal::{self, enable_raw_mode}, ExecutableCommand
};
use lazy_static::lazy_static;
use std::{
    io::{Stdout, stdout},
    sync::Mutex,
};

use crate::games::{Game, pong};

lazy_static! {
    pub static ref STD_OUT: Mutex<Stdout> = Mutex::new(stdout());
}
pub static BACKGROUND_COLOR: Mutex<Color> = Mutex::new(Color::Black);
pub static FOREGROUND_COLOR: Mutex<Color> = Mutex::new(Color::White);

mod games;
mod utils;

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = STD_OUT.lock().unwrap();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::Hide).unwrap();

    drop(stdout);

    pong::Pong.start().unwrap();
}
