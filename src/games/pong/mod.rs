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
        let (w, h) = terminal::size().unwrap();
        let (w, h) = (w as usize, h as usize);

        let state = PongState {
            player1: Player::new(5, (h / 2) - 5),
            player2: Player::new(w - 5, (h / 2) - 5),
            ball: Ball::new((w / 2) - 1, h / 2),
        };

        game_loop(state, game)
    }
}

struct PongState {
    player1: Player,
    player2: Player,
    ball: Ball,
}

struct Ball {
    rect: Rect,
    position: (f64, f64),
    movement: (f64, f64),
}

impl Ball {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            rect: Rect::new(
                x,
                y,
                2,
                1,
                Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            ),
            position: (x as f64, y as f64),
            movement: (-1., 0.5),
        }
    }

    pub const fn position(&self) -> (f64, f64) {
        self.position
    }

    pub const fn movement(&self) -> (f64, f64) {
        self.movement
    }

    pub const fn inverse_x_movement(&mut self) {
        self.movement.0 = -self.movement.0
    }

    pub fn update(&mut self, terminal_height: usize, elapsed_millis: u128) {
        self.position.0 += (self.movement.0 * elapsed_millis as f64) / 50.;
        self.position.1 += (self.movement.1 * elapsed_millis as f64) / 50.;

        if self.position.1 < 0. || self.position.1 + 1. > terminal_height as f64 {
            self.movement.1 = -self.movement.1;
        }

        self.rect
            .move_to(self.position.0 as usize, self.position.1 as usize);
    }
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
    internal
        .ball
        .update(state.terminal_height, state.delta_t.as_millis());

    if internal.ball.position().0 < 0. {
        return Some(Ok(&GameOver));
    } else if internal.ball.position().0 + 2. > state.terminal_width as f64 {
        return Some(Err(GameErr::NotYetImplemented));
    }

    if internal
        .player1
        .touches_ball(internal.ball.movement(), internal.ball.position(), true)
        || internal
            .player2
            .touches_ball(internal.ball.movement(), internal.ball.position(), false)
    {
        internal.ball.inverse_x_movement();
    }

    None
}
