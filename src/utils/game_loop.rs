use crate::games::{Game, GameErr};
use crossterm::{
    event::{KeyCode, poll},
    terminal,
};
use std::time::{Duration, Instant};

pub struct State {
    pub keyboard_state: Vec<KeyCode>,
    pub terminal_height: usize,
    pub terminal_width: usize,
    pub delta_t: Duration,
}

/// SINGLE THREAD basic game loop
pub fn game_loop<T>(
    mut internal_state: T,
    run: impl Fn(&State, &mut T) -> Option<Result<&'static dyn Game, GameErr>>,
) -> Result<&'static dyn Game, GameErr> {
    let (w, h) = terminal::size().unwrap();
    let mut state = State {
        keyboard_state: vec![],
        terminal_width: w as usize,
        terminal_height: h as usize,
        delta_t: Duration::from_millis(0),
    };
    let mut last_execution = Instant::now();

    loop {
        let (w, h) = terminal::size().unwrap();
        state.keyboard_state = vec![];

        while poll(Duration::from_millis(10)).unwrap_or(false) {
            if let Ok(event) = crossterm::event::read() {
                match event {
                    crossterm::event::Event::Key(key_event) => {
                        if !key_event.is_release() {
                            state.keyboard_state.push(key_event.code);
                        }
                    }
                    _ => (),
                }
            }
        }

        let elapsed = last_execution.elapsed();
        last_execution = Instant::now();

        state.delta_t = elapsed;
        state.terminal_height = h as usize;
        state.terminal_width = w as usize;

        if let Some(v) = run(&state, &mut internal_state) {
            return v;
        }
    }
}
