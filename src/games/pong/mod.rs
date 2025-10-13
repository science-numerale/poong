use crate::{
    games::{Game, GameErr},
    utils::{
        game_loop::{State, game_loop},
        rect::Rect,
    },
};
use crossterm::{event::KeyCode, style::Color};

pub struct Pong;

impl Game for Pong {
    fn start(&self) -> Result<&dyn Game, GameErr> {
        let player = Rect::new(
            1,
            0,
            2,
            10,
            Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        );
        let (x, y) = (5., 0.);
        let direction = Direction::None;

        let state = PongState {
            player,
            position: (x, y),
            direction,
        };

        game_loop(state, game)
    }
}

struct PongState {
    player: Rect,
    position: (f64, f64),
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    None,
}

fn game(state: &State, internal: &mut PongState) -> Option<Result<&'static dyn Game, GameErr>> {
    if state.keyboard_state.contains(&KeyCode::Up) {
        internal.direction = Direction::Up;
    }
    if state.keyboard_state.contains(&KeyCode::Down) {
        internal.direction = Direction::Down;
    }

    if internal.position.1 < 0. {
        internal.direction = Direction::Down;
    }
    if internal.position.1 + 9. > state.terminal_height as f64 {
        internal.direction = Direction::Up;
    }

    if internal.direction == Direction::Up {
        internal.position.1 -= (state.delta_t.as_millis() as f64) / 50.;
    }

    if internal.direction == Direction::Down {
        internal.position.1 += (state.delta_t.as_millis() as f64) / 50.;
    }

    internal.player.move_to(
        internal.position.0.trunc() as usize,
        internal.position.1.trunc() as usize,
    );

    None
}
