use crate::{
    games::{Game, GameErr, game_over::GameOver},
    utils::{
        game_loop::{State, game_loop},
        rect::Rect,
    },
};
use crossterm::{event::KeyCode, style::Color};

pub struct Pong;

impl Game for Pong {
    fn start(&self) -> Result<&dyn Game, GameErr> {
        let player1 = Rect::new(
            0,
            0,
            2,
            10,
            Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        );
        let player1_direction = PlayerDirection::None;
        let ball = Rect::new(
            0,
            0,
            2,
            1,
            Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        );
        let player2 = Rect::new(
            2,
            0,
            2,
            10,
            Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        );

        let state = PongState {
            player1,
            player1_position: (5., 0.),
            player1_direction: PlayerDirection::None,
            player2,
            player2_position: (95., 0.),
            player2_direction: PlayerDirection::None,
            ball,
            ball_position: (50., 10.),
            ball_movement: (-1., 0.5),
        };

        game_loop(state, game)
    }
}

struct PongState {
    player1: Rect,
    player1_position: (f64, f64),
    player1_direction: PlayerDirection,
    player2: Rect,
    player2_position: (f64, f64),
    player2_direction: PlayerDirection,
    ball: Rect,
    ball_position: (f64, f64),
    ball_movement: (f64, f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerDirection {
    Up,
    Down,
    None,
}

fn game(state: &State, internal: &mut PongState) -> Option<Result<&'static dyn Game, GameErr>> {
    if state.keyboard_state.contains(&KeyCode::Up) {
        internal.player1_direction = PlayerDirection::Up;
    }
    if state.keyboard_state.contains(&KeyCode::Down) {
        internal.player1_direction = PlayerDirection::Down;
    }
    if state.keyboard_state.contains(&KeyCode::Char('z')) {
        internal.player2_direction = PlayerDirection::Up;
    }
    if state.keyboard_state.contains(&KeyCode::Char('s')) {
        internal.player2_direction = PlayerDirection::Down;
    }
    if state.keyboard_state.contains(&KeyCode::Char('q')) {
        return Some(Err(GameErr::NotYetImplemented));
    }

    if internal.player1_position.1 < 0. {
        internal.player1_direction = PlayerDirection::Down;
    }
    if internal.player1_position.1 + 9. > state.terminal_height as f64 {
        internal.player1_direction = PlayerDirection::Up;
    }

    if internal.player2_position.1 < 0. {
        internal.player2_direction = PlayerDirection::Down;
    }
    if internal.player2_position.1 + 9. > state.terminal_height as f64 {
        internal.player2_direction = PlayerDirection::Up;
    }

    match internal.player1_direction {
        PlayerDirection::Up => {
            internal.player1_position.1 -= (state.delta_t.as_millis() as f64) / 50.
        }
        PlayerDirection::Down => {
            internal.player1_position.1 += (state.delta_t.as_millis() as f64) / 50.
        }
        _ => (),
    }

    match internal.player2_direction {
        PlayerDirection::Up => {
            internal.player2_position.1 -= (state.delta_t.as_millis() as f64) / 50.
        }
        PlayerDirection::Down => {
            internal.player2_position.1 += (state.delta_t.as_millis() as f64) / 50.
        }
        _ => (),
    }

    internal.ball_position.0 += (internal.ball_movement.0 * state.delta_t.as_millis() as f64) / 50.;
    internal.ball_position.1 += (internal.ball_movement.1 * state.delta_t.as_millis() as f64) / 50.;

    if internal.ball_position.0 < 0. {
        return Some(Ok(&GameOver));
    } else if internal.ball_position.0 + 2. > state.terminal_width as f64 {
        // TODO: win
        return Some(Err(GameErr::NotYetImplemented));
    }

    if internal.ball_position.1 < 0. || internal.ball_position.1 + 1. > state.terminal_height as f64
    {
        internal.ball_movement.1 = -internal.ball_movement.1;
    }

    if internal.ball_position.0.trunc() as usize == internal.player1.x() + 2
        && internal.ball_position.1.trunc() as usize > internal.player1.y()
        && (internal.ball_position.1.trunc() as usize) < internal.player1.y() + 20
        && internal.ball_movement.0 < 0.
    {
        internal.ball_movement.0 = -internal.ball_movement.0
    }

    if internal.ball_position.0.trunc() as usize == internal.player2.x() - 2
        && internal.ball_position.1.trunc() as usize > internal.player2.y()
        && (internal.ball_position.1.trunc() as usize) < internal.player2.y() + 20
        && internal.ball_movement.0 > 0.
    {
        internal.ball_movement.0 = -internal.ball_movement.0
    }

    internal.player1.move_to(
        internal.player1_position.0.trunc() as usize,
        internal.player1_position.1.trunc() as usize,
    );

    internal.player2.move_to(
        internal.player2_position.0.trunc() as usize,
        internal.player2_position.1.trunc() as usize,
    );

    internal.ball.move_to(
        internal.ball_position.0.trunc() as usize,
        internal.ball_position.1.trunc() as usize,
    );

    None
}
