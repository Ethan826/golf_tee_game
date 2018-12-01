use crate::{is_triangular, GameError};

/// Represents the state of a game.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GameState(Vec<bool>);

impl GameState {
    /// Given a `Vec` of `bool`, return an instance of `GameState`.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of `input` is not a triangular number.
    /// Also returns an error if the game state is invalid because all spaces are
    /// occupied.
    pub fn new(input: Vec<bool>) -> Result<Self, GameError> {
        if is_triangular(input.len()) {
            if input.iter().all(|pos| *pos) {
                Err(GameError::InvalidGameState)
            } else {
                Ok(GameState(input))
            }
        } else {
            Err(GameError::InvalidGameSize)
        }
    }

    /// Return whether the specified position is occupied.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified position is invalid.
    pub fn is_occupied(&self, position: usize) -> Result<bool, GameError> {
        match self.0.get(position) {
            Some(result) => Ok(*result),
            None => Err(GameError::InvalidPosition),
        }
    }

    /// Removes a tee from the specified position, mutating the game state.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified position is invalid or unoccupied.
    pub fn remove_tee(&mut self, position: usize) -> Result<(), GameError> {
        match self.0.get(position) {
            Some(value) => {
                if *value {
                    self.0[position] = false;
                    Ok(())
                } else {
                    Err(GameError::InvalidMove)
                }
            }
            None => Err(GameError::InvalidMove),
        }
    }

    /// Return the number of positions on the game board.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_game_state_new_valid() {
    let state = GameState::new(vec![true, false, true]);
    assert!(state.is_ok());
}

#[test]
fn test_game_state_new_invalid_not_triangular() {
    let state = GameState::new(vec![true, false, true, false]);
    assert!(state.is_err());
}

#[test]
fn test_game_state_new_invalid_all_full() {
    let state = GameState::new(vec![true, true, true]);
    assert!(state.is_err());
}

#[test]
fn test_game_state_position_is_occupied_valid() {
    let state = GameState::new(vec![true, false, true]).unwrap();
    assert_eq!(state.is_occupied(0).unwrap(), true);
}

#[test]
fn test_game_state_position_is_occupied_invalid() {
    let state = GameState::new(vec![true, false, true]).unwrap();
    assert!(state.is_occupied(3).is_err());
}

#[test]
fn test_game_state_remove_tee_valid() {
    let mut state = GameState::new(vec![true, false, true]).unwrap();
    assert!(state.is_occupied(0).unwrap());
    assert!(state.remove_tee(0).is_ok());
    assert!(!state.is_occupied(0).unwrap());
}

#[test]
fn test_game_state_remove_tee_invalid() {
    let mut state = GameState::new(vec![false, false, true]).unwrap();
    assert!(state.remove_tee(0).is_err());
}
