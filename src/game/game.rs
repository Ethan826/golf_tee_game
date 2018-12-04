use super::game_state::GameState;
use super::legal_moves::LegalMoves;
use super::position_data::PositionData;
use crate::GameError;

/// Represents a game's state and that game's rules.
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
    /// Returns an error if the length of the `state` and `legal_moves`
    /// collections are different.
    pub fn new(state: GameState, legal_moves: LegalMoves) -> Result<Self, GameError> {
        if legal_moves.len() == state.len() {
            Ok(Game { legal_moves, state })
        } else {
            Err(GameError::InvalidGameSize)
        }
    }

    /// Returns whether a particular game position is occupied.
    ///
    /// # Errors
    ///
    /// Returns an error if the position is not on the game board.
    pub fn is_occupied(&self, position: usize) -> Result<bool, GameError> {
        self.state.is_occupied(position)
    }

    fn available_moves(&self) {
        self.state.iter();
    }

    fn available_moves_from_position(
        &self,
        position: usize,
    ) -> Result<Option<&PositionData>, GameError> {
        if self.is_occupied(position)? {
            Ok(Some(self.legal_moves.position_data(position)?))
        } else {
            Ok(None)
        }
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_game_new_valid() {
    use super::standard_game::STANDARD_MOVES;

    let moves = LegalMoves::new(STANDARD_MOVES.to_vec()).unwrap();
    let state = GameState::new((0..moves.len()).map(|_| false).collect()).unwrap();

    let subject = Game::new(state, moves);
    assert!(subject.is_ok());
}

#[test]
fn test_game_new_invalid() {
    use super::standard_game::STANDARD_MOVES;

    let state = GameState::new(vec![false, true, true]).unwrap();
    let moves = LegalMoves::new(STANDARD_MOVES.to_vec()).unwrap();

    let subject = Game::new(state, moves);
    assert!(subject.is_err());
}