use crate::{
    games::{Game, GameErr, game_over::GameOver},
    utils::{
        game_loop::{State, game_loop},
        rect::Rect,
    },
};
use crossterm::{event::KeyCode, style::Color, terminal};

pub struct Pong;

impl Game for Pong {
    fn start(&self) -> Result<&dyn Game, GameErr> {
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

        let (w, h) = terminal::size().unwrap();
        let (w, h) = (w as usize, h as usize);

        let state = PongState {
            player1: Player::new(5, (h / 2) - 5),
            player2: Player::new(w - 5, (h / 2) - 5),
            ball,
            ball_position: ((((w as f64) / 2.) - 1.), (h as f64) / 2.),
            ball_movement: (-1., 0.5),
        };

        game_loop(state, game)
    }
}

struct PongState {
    player1: Player,
    player2: Player,
    ball: Rect,
    ball_position: (f64, f64),
    ball_movement: (f64, f64),
}

struct Player {
    rect: Rect,
    position: (f64, f64),
    direction: PlayerDirection,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            rect: Rect::new(
                x,
                y,
                2,
                10,
                Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            ),
            position: (x as f64, y as f64),
            direction: PlayerDirection::None,
        }
    }

    pub fn update(&mut self, terminal_height: usize, elapsed_millis: u128) {
        if self.position.1 < 0. {
            self.go_down();
        } else if self.position.1 + 9. > terminal_height as f64 {
            self.go_up();
        }

        match self.direction {
            PlayerDirection::Up => self.position.1 -= (elapsed_millis as f64) / 50.,
            PlayerDirection::Down => self.position.1 += (elapsed_millis as f64) / 50.,
            _ => (),
        }

        self.rect.move_to(
            self.position.0.trunc() as usize,
            self.position.1.trunc() as usize,
        );
    }

    pub fn touches_ball(
        &self,
        ball_movement: (f64, f64),
        ball_position: (f64, f64),
        from_right: bool,
    ) -> bool {
        ball_position.0.trunc() as usize == {
            if from_right {
                self.rect.x() + 2
            } else {
                self.rect.x() - 2
            }
        } && ball_position.1.trunc() as usize > self.rect.y()
            && (ball_position.1.trunc() as usize) < self.rect.y() + 10
            && ((ball_movement.0 > 0.) ^ from_right)
    }

    pub fn go_up(&mut self) {
        self.direction = PlayerDirection::Up
    }

    pub fn go_down(&mut self) {
        self.direction = PlayerDirection::Down
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerDirection {
    Up,
    Down,
    None,
}

fn game(state: &State, internal: &mut PongState) -> Option<Result<&'static dyn Game, GameErr>> {
    for k in &state.keyboard_state {
        match k {
            KeyCode::Up => internal.player2.go_up(),
            KeyCode::Down => internal.player2.go_down(),
            KeyCode::Char('z') => internal.player1.go_up(),
            KeyCode::Char('s') => internal.player1.go_down(),
            KeyCode::Char('q') => return Some(Err(GameErr::NotYetImplemented)),
            _ => (),
        }
    }

    internal
        .player1
        .update(state.terminal_height, state.delta_t.as_millis());
    internal
        .player2
        .update(state.terminal_height, state.delta_t.as_millis());

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

    if internal
        .player1
        .touches_ball(internal.ball_movement, internal.ball_position, true)
        || internal
            .player2
            .touches_ball(internal.ball_movement, internal.ball_position, false)
    {
        internal.ball_movement.0 = -internal.ball_movement.0
    }

    internal.ball.move_to(
        internal.ball_position.0.trunc() as usize,
        internal.ball_position.1.trunc() as usize,
    );

    None
}
