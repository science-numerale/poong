use crate::games::Game;

pub struct GameOver;

impl Game for GameOver {
    fn start(&self) -> Result<&dyn Game, super::GameErr> {
        Err(super::GameErr::NotYetImplemented)
    }
}
