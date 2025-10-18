use std::{
    fmt::Display,
    io::{Stdout, Write},
    ops::Range,
    time::Duration,
};

use crate::{
    games::{TickedGame, TickedGameFeedback, TickedGameUpdate},
    utils::math::Vector2,
};
use crossterm::{
    QueueableCommand, cursor,
    event::{KeyCode, KeyEvent, KeyEventKind},
    style::{self, Color, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear},
};
use rand::random_range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum PlayerDirection {
    Up,
    #[default]
    Down,
}

#[derive(Debug)]
struct Player {
    position: f64,
    direction: PlayerDirection,
    margin: f64,
    size: f64,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: 0.5,
            direction: PlayerDirection::Down,
            margin: 0.1,
            size: 0.25,
        }
    }
}

impl Player {
    fn get_range(&self) -> Range<f64> {
        self.position..(self.position + self.size)
    }
    fn tick(&mut self, duration: Duration, speed: f64) {
        let offset = speed * duration.as_secs_f64();
        match self.direction {
            PlayerDirection::Up => self.position -= offset,
            PlayerDirection::Down => self.position += offset,
        }

        let range = self.get_range();
        if range.start < 0. - self.margin {
            self.position = -self.margin;
            self.direction = PlayerDirection::Down;
        } else if range.end > 1. + self.margin {
            self.position = 1. + self.margin - self.size;
            self.direction = PlayerDirection::Up;
        }
    }
}

#[derive(Debug)]
pub enum PongEndReason {
    Exited,
    Out,
}

impl Display for PongEndReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Exited => "vous avez appuyé sur Échap",
                Self::Out => "la balle est sortie du terrain",
            }
        )
    }
}

#[derive(Debug)]
pub struct PongResult {
    pub bounces: usize,
    pub reason: PongEndReason,
}

impl Display for PongResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "La balle a rebondit {} fois et la partie s'est terminée car {} !",
            self.bounces, self.reason
        )
    }
}

struct Ball {
    position: Vector2<f64>,
    movement: Vector2<f64>,
}

impl Ball {
    fn tick(&mut self, duration: Duration) {
        self.position = self.position + self.movement * duration.as_secs_f64().into();
        if self.position.1 < 0. {
            self.movement.1 = self.movement.1.abs();
        } else if self.position.1 > 1. {
            self.movement.1 = -self.movement.1.abs();
        }
    }

    fn faster(&mut self) {
        let offset = Vector2(random_range(0.0..0.05), random_range(-0.5..0.5));

        if self.movement.0 > 0. {
            self.movement = self.movement + offset;
        } else if self.movement.0 < 0. {
            self.movement = self.movement + offset * Vector2(-1., 1.);
        }
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            position: Vector2(0.5, 0.5),
            movement: Vector2(-0.15, -0.05),
        }
    }
}

pub struct Pong<'stdout> {
    players: (Player, Player),
    ball: Ball,
    bounces: usize,

    stdout: &'stdout Stdout,
}

impl Pong<'_> {
    const fn get_result(&self, reason: PongEndReason) -> PongResult {
        PongResult {
            bounces: self.bounces,
            reason,
        }
    }
}

impl<'stdout> From<&'stdout Stdout> for Pong<'stdout> {
    fn from(value: &'stdout Stdout) -> Self {
        Self {
            players: Default::default(),
            ball: Ball::default(),
            bounces: 0,
            stdout: value,
        }
    }
}

impl TickedGame<PongResult, TickedGameUpdate> for Pong<'_> {
    #[allow(clippy::too_many_lines)]
    fn tick(&mut self, update: TickedGameUpdate) -> TickedGameFeedback<PongResult> {
        for event in update.events {
            if let crossterm::event::Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) = event
            {
                match code {
                    KeyCode::Char('z' | 'k') => {
                        self.players.0.direction = PlayerDirection::Up;
                    }
                    KeyCode::Char('s' | 'j') => {
                        self.players.0.direction = PlayerDirection::Down;
                    }
                    KeyCode::Up | KeyCode::Char('l') => {
                        self.players.1.direction = PlayerDirection::Up;
                    }
                    KeyCode::Down | KeyCode::Char('h') => {
                        self.players.1.direction = PlayerDirection::Down;
                    }
                    KeyCode::Esc => {
                        return TickedGameFeedback::End(self.get_result(PongEndReason::Exited));
                    }
                    _ => {}
                }
            }
        }

        self.players
            .0
            .tick(update.elapsed.unwrap_or(Duration::ZERO), 0.5);
        self.players
            .1
            .tick(update.elapsed.unwrap_or(Duration::ZERO), 0.5);
        self.ball.tick(update.elapsed.unwrap_or(Duration::ZERO));

        let player_margin = 0.05;

        if self.ball.position.0 <= 0. + player_margin {
            if self.players.0.get_range().contains(&self.ball.position.1) {
                self.ball.faster();
                self.ball.movement.0 = self.ball.movement.0.abs();
                self.bounces += 1;
            } else {
                return TickedGameFeedback::End(self.get_result(PongEndReason::Out));
            }
        } else if self.ball.position.0 >= 1. - player_margin {
            if self.players.1.get_range().contains(&self.ball.position.1) {
                self.ball.faster();
                self.ball.movement.0 = -self.ball.movement.0.abs();
                self.bounces += 1;
            } else {
                return TickedGameFeedback::End(self.get_result(PongEndReason::Out));
            }
        }

        {
            let window_size: Vector2<u16> = terminal::size().unwrap().into();
            let window_size = window_size.map(|x| f64::from(*x)) * Vector2(0.5, 1.0);
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let window_size: Vector2<u16> =
                window_size.map(|x| (x.abs() as u64).try_into().unwrap());

            self.stdout.queue(Clear(terminal::ClearType::All)).unwrap();

            // TODO: remove duplicated code
            let mut draw_at = |c: [char; 2], pos: Vector2<u16>, colors: Option<(Color, Color)>| {
                if let Some(colors) = colors {
                    self.stdout.queue(SetForegroundColor(colors.0)).unwrap();
                    self.stdout.queue(SetBackgroundColor(colors.1)).unwrap();
                }

                self.stdout
                    .queue(cursor::MoveTo(pos.0 * 2, pos.1))
                    .unwrap()
                    .queue(style::Print(c[0].to_string() + &c[1].to_string()))
                    .unwrap();

                if colors.is_some() {
                    self.stdout
                        .queue(SetForegroundColor(style::Color::Reset))
                        .unwrap();
                    self.stdout
                        .queue(SetBackgroundColor(style::Color::Reset))
                        .unwrap();
                }
            };

            draw_at(
                [' '; 2],
                #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                (self.ball.position * window_size.map(|x| f64::from(*x))).map(|x| x.floor() as u16),
                Some((Color::Reset, Color::Red)),
            );

            {
                let int_range = self.players.0.get_range();
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let int_range: Range<u16> = (int_range.start * f64::from(window_size.1)).floor()
                    as u16
                    ..(int_range.end * f64::from(window_size.1)).floor() as u16;

                for y in int_range {
                    draw_at(
                        ['a'; 2],
                        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                        Vector2(
                            (f64::from(window_size.0) * player_margin).round().abs() as u16 - 1,
                            y,
                        ),
                        Some((Color::White, Color::White)),
                    );
                }
            }

            {
                let int_range = self.players.1.get_range();
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let int_range: Range<u16> = (int_range.start * f64::from(window_size.1)).floor()
                    as u16
                    ..(int_range.end * f64::from(window_size.1)).floor() as u16;

                for y in int_range {
                    draw_at(
                        ['a'; 2],
                        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                        Vector2(
                            (f64::from(window_size.0) * (1. - player_margin))
                                .round()
                                .abs() as u16,
                            y,
                        ),
                        Some((Color::White, Color::White)),
                    );
                }
            }

            self.stdout.flush().unwrap();
        }

        TickedGameFeedback::Next(Duration::from_millis(10))
    }
}
