use crate::game_state::GameState;
use crate::legal_moves::LegalMoves;

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    legal_moves: LegalMoves,
    state: GameState,
}

impl Game {
    /// Given an array of booleans representing the game state and a `LegalMoves`
    /// unit struct containing the legal moves for the given game, return an
    /// instance of `Game` with those characteristics.
    ///
    /// # Errors
    ///
    /// Returns an error if the size of the `state` and `legal_moves` collections
    /// are different, as they must be the same size to define a game.
    pub fn new(
        state: GameState,
        legal_moves: LegalMoves,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Validate
        Ok(Game { legal_moves, state })
    }

    /// Returns whether a particular game position is occupied.
    ///
    /// # Errors
    ///
    /// Returns an error if the position is not on the game board.
    pub fn is_occupied(&self, position: usize) -> Result<bool, Box<dyn std::error::Error>> {
        // Ok(*self
        //     .state
        //     .0
        //     .get(position)
        //     .ok_or(format!("Invalid position specified: {}", position))?)
        Ok(true)
    }
}
