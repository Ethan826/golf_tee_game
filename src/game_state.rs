/// Represents the state of a game.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GameState(Vec<bool>);

impl GameState {
    pub fn new(Vec<bool>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(GameState(Vec<bool>))
    }
}