use std::{
    fmt::Display,
    io::{Stdout, Write},
    ops::Neg,
    thread,
    time::Duration,
};

use crossterm::{
    QueueableCommand, cursor,
    event::{KeyCode, KeyEvent, KeyEventKind},
    style::{self, Color, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use rand::random_range;

use crate::{
    games::{TickedGame, TickedGameFeedback, TickedGameUpdate},
    utils::math::Vector2,
};

#[derive(Debug)]
pub enum SnakeEndReason {
    Crash,
    WindowResized,
    Exited,
    Overkill,
}

impl Display for SnakeEndReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Crash => "le serpent s'est mordu la queue",
                Self::WindowResized => "la fenêtre a été redimensionnée",
                Self::Exited => "vous avez appuyé sur Échap",
                Self::Overkill => "... n'avez-vous pas *triché* ? Votre serpent remplit TOUT",
            }
        )
    }
}

#[derive(Debug)]
pub struct SnakeResult {
    pub length: usize,
    pub reason: SnakeEndReason,
}

impl Display for SnakeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Le serpend a atteint {} carreaux et la partie s'est terminée car {} !",
            self.length, self.reason
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Debug)]
pub struct Snake<'stdout> {
    body: Vec<Vector2<u16>>, // La queue à au début, la tête à la fin
    direction: Direction,

    apple: Vector2<u16>,

    window_size: Option<Vector2<u16>>,

    stdout: &'stdout Stdout,
}

impl Snake<'_> {
    const fn get_result(&self, reason: SnakeEndReason) -> SnakeResult {
        SnakeResult {
            length: self.body.len(),
            reason,
        }
    }
}

impl<'stdout> From<&'stdout Stdout> for Snake<'stdout> {
    fn from(value: &'stdout Stdout) -> Self {
        Snake {
            body: vec![Vector2(0, 0)],
            direction: Direction::Right,
            stdout: value,
            window_size: None,
            apple: Vector2(3, 3),
        }
    }
}

// Je vais importer un module de math pour faire plus proprement après, mais c'est assez bien pour l'instant

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
impl TickedGame<SnakeResult, TickedGameUpdate> for Snake<'_> {
    #[allow(clippy::too_many_lines)]
    fn tick(&mut self, update: TickedGameUpdate) -> TickedGameFeedback<SnakeResult> {
        #[allow(clippy::cast_sign_loss)]
        let adapt_size =
            |size: &Vector2<u16>| Vector2((f32::from(size.0) / 2.).floor() as u16, size.1);
        if self.window_size.is_none() {
            self.window_size = Some(adapt_size(&terminal::size().unwrap().into()));
        }
        // Ne peut pas être None
        let window_size = self.window_size.unwrap();

        let mut direction = self.direction;
        let mut set_direction = |dir: Direction| {
            if dir != -self.direction {
                direction = dir;
            }
        };

        for event in update.events {
            match event {
                crossterm::event::Event::Key(KeyEvent {
                    code,
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => match code {
                    KeyCode::Up | KeyCode::Char('k') => set_direction(Direction::Up),
                    KeyCode::Right | KeyCode::Char('l') => set_direction(Direction::Right),
                    KeyCode::Down | KeyCode::Char('j') => set_direction(Direction::Down),
                    KeyCode::Left | KeyCode::Char('h') => set_direction(Direction::Left),
                    KeyCode::Esc => {
                        return TickedGameFeedback::End(self.get_result(SnakeEndReason::Exited));
                    }
                    _ => {}
                },
                crossterm::event::Event::Resize(x, y) => {
                    if window_size != adapt_size(&Vector2(x, y)) {
                        return TickedGameFeedback::End(
                            self.get_result(SnakeEndReason::WindowResized),
                        );
                    }
                }
                _ => {}
            }
        }

        self.direction = direction;

        let head_cell = self.body.last().unwrap(); // Ne peut pas être vide
        let hc = head_cell; // Abréviation

        let hc = (hc.0.cast_signed(), hc.1.cast_signed());

        let next_cell: Vector2<i16> = (match self.direction {
            Direction::Up => (hc.0, hc.1 - 1),
            Direction::Right => (hc.0 + 1, hc.1),
            Direction::Down => (hc.0, hc.1 + 1),
            Direction::Left => (hc.0 - 1, hc.1),
        })
        .into();
        let next_cell = Vector2(
            (next_cell.0 + window_size.0.cast_signed()).cast_unsigned() % window_size.0,
            (next_cell.1 + window_size.1.cast_signed()).cast_unsigned() % window_size.1,
        );

        for pos in self.body.clone() {
            if pos == next_cell {
                thread::sleep(Duration::from_secs(1));
                return TickedGameFeedback::End(self.get_result(SnakeEndReason::Crash));
            }
        }

        self.body.push(next_cell);
        if next_cell == self.apple {
            let space = window_size.0 * window_size.1 - self.body.len() as u16;
            let chosen = random_range(0..space);

            let pos = (0..window_size.1)
                .flat_map(|y| (0..window_size.0).map(move |x| Vector2(x, y)))
                .filter(|pos| !self.body.contains(pos))
                .nth(chosen.into());

            if let Some(pos) = pos {
                self.apple = pos;
            } else {
                return TickedGameFeedback::End(self.get_result(SnakeEndReason::Overkill));
            }
        } else {
            // TODO: optimiser ça
            self.body.remove(0);
        }

        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        let mut draw_at = |c: [char; 2], pos: &Vector2<u16>, colors: Option<(Color, Color)>| {
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
            &self.apple,
            Some((style::Color::Green, style::Color::Red)),
        );

        for (i, pos) in self.body.iter().enumerate() {
            draw_at(
                [if i == self.body.len() - 1 { '•' } else { ' ' }; 2],
                pos,
                Some((style::Color::Black, style::Color::Green)),
            );
        }

        self.stdout.flush().unwrap();

        let length = self.body.len() as f32;
        TickedGameFeedback::Next(Duration::from_secs_f32(
            1. / 0.1f32.mul_add(length.powi(2), 4.),
        ))
    }
}
