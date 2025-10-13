pub mod pong;
pub mod game_over;

pub trait Game {
    fn start(&self) -> Result<&dyn Game, GameErr>;
}

#[derive(Debug)]
pub enum GameErr {
    NotYetImplemented,
}
