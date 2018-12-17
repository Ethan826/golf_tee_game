use crate::game::game_move::GameMove;
use crate::{is_triangular, GameError};

/// Represents the state of a game.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GameState {
    state: Vec<bool>,
    filled: usize,
}

impl GameState {
    /// Given a `Vec` of `bool`, return an instance of `GameState`.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of `input` is not a triangular number.
    /// Also returns an error if the game state is invalid because all spaces are
    /// occupied.
    pub fn new(state: Vec<bool>) -> Result<Self, GameError> {
        if is_triangular(state.len()) {
            match GameState::filled_positions(&state) {
                filled if state.len() == filled => Err(GameError::InvalidGameState),
                filled => Ok(GameState { state, filled }),
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
        match self.state.get(position) {
            Some(result) => Ok(*result),
            None => Err(GameError::InvalidPosition),
        }
    }

    /// Plays the specified move, returning a copy of the game state with the
    /// move having been played.
    ///
    /// # Errors
    ///
    /// Returns an error if the move is invalid.
    pub fn make_move(&self, game_move: &GameMove) -> Result<Self, GameError> {
        match (
            self.is_occupied(game_move.starting_space)?,
            self.is_occupied(game_move.leapt_space)?,
            self.is_occupied(game_move.destination_space)?,
        ) {
            (true, true, false) => Ok({
                let mut copy = self.to_owned();
                copy.state[game_move.starting_space] = false;
                copy.state[game_move.leapt_space] = false;
                copy.state[game_move.destination_space] = true;
                copy.filled -= 1;
                copy
            }),
            _ => Err(GameError::InvalidMove),
        }
    }

    /// Removes a tee from the specified position, mutating the game state.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified position is invalid or unoccupied.
    pub fn remove_tee(&self, position: usize) -> Result<Self, GameError> {
        match self.state.get(position) {
            Some(value) => {
                if *value {
                    let mut copy = self.to_owned();
                    copy.state[position] = false;
                    copy.filled -= 1;
                    Ok(copy)
                } else {
                    Err(GameError::InvalidMove)
                }
            }
            None => Err(GameError::InvalidMove),
        }
    }

    /// Removes a tee from the specified position, mutating the game state.
    ///
    /// # Errors
    ///
    /// Returns an error if the specified position is invalid or unoccupied.
    pub fn insert_tee(&self, position: usize) -> Result<Self, GameError> {
        match self.state.get(position) {
            Some(value) => {
                if *value {
                    Err(GameError::InvalidMove)
                } else {
                    let mut copy = self.to_owned();
                    copy.state[position] = true;
                    copy.filled += 1;
                    Ok(copy)
                }
            }
            None => Err(GameError::InvalidMove),
        }
    }

    /// Return the number of positions on the game board.
    pub fn len(&self) -> usize {
        self.state.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, bool> {
        self.state.iter()
    }

    /// Return the number of positions filled.
    pub fn filled(&self) -> usize {
        self.filled
    }

    fn filled_positions(state: &[bool]) -> usize {
        state.iter().filter(|pos| **pos).count()
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
    assert_eq!(state.filled, 2);
    assert!(state.is_occupied(0).unwrap());

    let after_move = state.remove_tee(2).unwrap();
    assert!(!after_move.is_occupied(2).unwrap());
    assert_eq!(after_move.filled, 1);
}

#[test]
fn test_game_state_remove_tee_invalid() {
    let mut state = GameState::new(vec![false, false, true]).unwrap();
    assert!(state.remove_tee(0).is_err());
}

#[test]
fn test_game_state_insert_tee_valid() {
    let state = GameState::new(vec![true, false, true]).unwrap();
    assert_eq!(state.filled, 2);
    assert!(!state.is_occupied(1).unwrap());

    let after_move = state.insert_tee(1).unwrap();
    assert!(after_move.is_occupied(1).unwrap());
    assert_eq!(after_move.filled, 3);
}

#[test]
fn test_game_state_insert_tee_invalid() {
    let mut state = GameState::new(vec![false, false, true]).unwrap();
    assert!(state.insert_tee(3).is_err());
}
