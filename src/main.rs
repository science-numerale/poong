#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveTo},
    terminal,
};
use std::{
    fmt::Display,
    io::{self, Write, stdin, stdout},
    thread,
    time::Duration,
};

mod utils;

use crate::games::{Game, pong::Pong, snake::Snake};

mod games;

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::Clear(terminal::ClearType::All))?
        .flush()?;

    loop {
        macro_rules! play_game {
            ($game:ty) => {{
                stdout
                    .queue(terminal::EnterAlternateScreen)?
                    .queue(cursor::Hide)?
                    .queue(terminal::Clear(terminal::ClearType::All))?
                    .flush()?;

                terminal::enable_raw_mode().unwrap();

                let res = <$game>::start(&stdout);

                stdout
                    .queue(terminal::LeaveAlternateScreen)?
                    .queue(cursor::Show)?
                    .queue(terminal::Clear(terminal::ClearType::All))?
                    .flush()?;

                terminal::disable_raw_mode().unwrap();

                Some(Box::new(res))
            }};
        }
        println!("À quel jeu voulez-vous jouer ? (pong/snake/quitter)");
        print!(">> ");
        stdout.flush()?;
        let mut answer = String::new();
        stdin().read_line(&mut answer).unwrap();
        let answer = answer.trim();

        let result: Option<Box<dyn Display>> = match answer {
            "pong" => play_game!(Pong),
            "snake" => play_game!(Snake),

            "quitter" => {
                println!("À bientôt !");
                break;
            }

            game => {
                println!("Le jeu {game} n'existe pas.");
                None
            }
        };

        if let Some(result) = result {
            stdout.queue(MoveTo(0, 0))?;
            println!("Résultat de la partie :");
            println!("{result}");
            println!("*appuyez sur Enter*");
            stdin().read_line(&mut String::new())?;
            stdout
                .queue(terminal::Clear(terminal::ClearType::All))?
                .queue(cursor::MoveTo(0, 0))?;
        }
    }

    stdout.execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}
