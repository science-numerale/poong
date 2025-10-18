use std::{thread, time::Duration};

use crossterm::event::{self, Event};

pub mod pong;
pub mod snake;

// Les paramètres Input et Output permettent d'utiliser un jeu dans un autre jeu, par example pour dire qu'un jeu doit retourner un score (pour utiliser ce même score dans le jeu plus général)
// Output pourra typiquement implémenter ClassicGameResult
pub trait Game<Input, Output> {
    fn start(input: Input) -> Output;
}

// À voir, pourra être utile plus tard, qui sait...
// trait ClassicGameResult {
//     fn get_earned(&self) -> isize; // Négatif si la partie est perdue, positif si la partie est gagnée
// }

// === TickedGame ===

struct TickedGameUpdate {
    events: Vec<Event>,
    elapsed: Option<Duration>,
}

enum TickedGameFeedback<Output> {
    Next(Duration), // Demande un nouveau tick dans un certain nombre de ms
    End(Output),    // Termine la partie et donne le résultat
}

trait TickedGame<Output, Update> {
    fn tick(&mut self, update: Update) -> TickedGameFeedback<Output>;
}

impl<Input, Output, G: TickedGame<Output, TickedGameUpdate> + From<Input>> Game<Input, Output>
    for G
{
    fn start(input: Input) -> Output {
        let mut game: Self = input.into();

        let mut elapsed = None;

        loop {
            let mut events = vec![];

            while event::poll(Duration::ZERO).unwrap_or_default() {
                if let Ok(event) = event::read() {
                    events.push(event);
                }
            }

            // FIXME: gérer les évenements
            match game.tick(TickedGameUpdate { events, elapsed }) {
                TickedGameFeedback::Next(duration) => {
                    thread::sleep(duration);
                    elapsed = Some(duration);
                }
                TickedGameFeedback::End(output) => return output,
            }
        }
    }
}
