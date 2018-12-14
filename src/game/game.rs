use super::game_move::GameMove;
use super::game_state::GameState;
use super::legal_moves::LegalMoves;
use crate::GameError;
use std::collections::HashSet;

/// Represents a game's state and that game's rules.
#[derive(Debug, PartialEq, Eq)]
pub struct Game<'a> {
    legal_moves: &'a LegalMoves,
    state: &'a GameState,
}

impl<'a> Game<'a> {
    /// Given an array of booleans representing the game state and a `LegalMoves`
    /// unit struct containing the legal moves for the given game, return an
    /// instance of `Game` with those characteristics.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of the `state` and `legal_moves`
    /// collections are different.
    pub fn new(state: &'a GameState, legal_moves: &'a LegalMoves) -> Result<Self, GameError> {
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

    /// Returns a hashset of game moves that are legal from the given position.
    /// An empty hashset signifies no available moves.
    ///
    /// # Errors
    ///
    /// Returns an error if the position is not on the game board.
    pub fn available_moves_from_position(
        &self,
        position: usize,
    ) -> Result<HashSet<&GameMove>, GameError> {
        if self.is_occupied(position)? {
            let game_moves = self.legal_moves.position_data(position)?;

            game_moves
                .iter()
                .filter_map(|game_move| match self.is_legal_move(game_move) {
                    Ok(true) => Some(Ok(game_move)),
                    Ok(false) => None,
                    Err(e) => Some(Err(e)),
                })
                .collect()
        } else {
            Ok(HashSet::with_capacity(0))
        }
    }

    /// Get all the moves that can be made in the current game state.
    ///
    /// # Errors
    ///
    /// Returns an error if iterating over the board and checking for moves at
    /// each position returns an error.
    pub fn all_available_moves(&self) -> Result<HashSet<&GameMove>, GameError> {
        self.state.iter().enumerate().try_fold(
            HashSet::with_capacity(self.state.len()),
            |acc, (position, is_occupied)| {
                if *is_occupied {
                    let moves = self.available_moves_from_position(position)?;
                    Ok(acc.union(&moves).cloned().collect())
                } else {
                    Ok(acc)
                }
            },
        )
    }

    fn is_legal_move(&self, game_move: &GameMove) -> Result<bool, GameError> {
        Ok(self.state.is_occupied(game_move.starting_space)?
            && self.state.is_occupied(game_move.leapt_space)?
            && !self.state.is_occupied(game_move.destination_space)?)
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[cfg(test)]
use crate::game::standard_game::*;

#[test]
fn test_game_new_valid() {
    let moves = LegalMoves::new(STANDARD_MOVES.to_vec()).unwrap();
    let state = GameState::new((0..moves.len()).map(|_| false).collect()).unwrap();

    let subject = Game::new(&state, &moves);
    assert!(subject.is_ok());
}

#[test]
fn test_game_new_invalid() {
    let state = GameState::new(vec![false, true, true]).unwrap();
    let moves = LegalMoves::new(STANDARD_MOVES.to_vec()).unwrap();

    let subject = Game::new(&state, &moves);
    assert!(subject.is_err());
}

#[test]
fn test_available_moves_from_unoccupied_position() {
    let (game_state, legal_moves) = build_standard_game_args(0);
    let game = Game::new(&game_state, &legal_moves).unwrap();
    let subject = game.available_moves_from_position(0).unwrap();

    assert_eq!(subject, hashset![]);
}

#[test]
fn test_available_moves_from_occupied_position_with_one_move() {
    let (game_state, legal_moves) = build_standard_game_args(1);
    let game = Game::new(&game_state, &legal_moves).unwrap();
    let subject = game.available_moves_from_position(6).unwrap();

    assert_eq!(
        subject,
        hashset! {&GameMove {
            starting_space: 6,
            leapt_space: 3,
            destination_space: 1
        }}
    );
}

#[test]
fn test_available_moves_from_occupied_position_with_multiple_moves() {
    let (mut game_state, legal_moves) = build_standard_game_args(3);
    game_state.remove_tee(5).ok();

    let game = Game::new(&game_state, &legal_moves).unwrap();
    let subject = game.available_moves_from_position(0).unwrap();

    assert_eq!(
        subject,
        hashset! {
            &GameMove {
                starting_space: 0,
                leapt_space: 1,
                destination_space: 3
            },
            &GameMove {
                starting_space: 0,
                leapt_space: 2,
                destination_space: 5
            }
        }
    );
}

#[test]
fn test_all_available_moves() {
    let (mut game_state, legal_moves) = build_standard_game_args(3);
    game_state.remove_tee(5).ok();

    let game = Game::new(&game_state, &legal_moves).unwrap();
    let subject = game.all_available_moves().unwrap();

    assert_eq!(
        subject,
        hashset! {
            &GameMove {
                starting_space: 0,
                leapt_space: 2,
                destination_space: 5
            },
            &GameMove {
                starting_space: 14,
                leapt_space: 9,
                destination_space: 5
            },
            &GameMove {
                starting_space: 12,
                leapt_space: 8,
                destination_space: 5
            },
            &GameMove {
                starting_space: 12,
                leapt_space: 7,
                destination_space: 3
            },
            &GameMove {
                starting_space: 0,
                leapt_space: 1,
                destination_space: 3
            },
            &GameMove {
                starting_space: 10,
                leapt_space: 6,
                destination_space: 3
            },
        }
    );
}

#[test]
fn test_all_available_moves_no_moves() {
    let mut game_state = GameState::new((0..15).map(|_| false).collect::<Vec<_>>()).unwrap();
    game_state.insert_tee(0).ok();
    game_state.insert_tee(4).ok();
    game_state.insert_tee(12).ok();
    game_state.insert_tee(14).ok();
    let legal_moves = LegalMoves::new(STANDARD_MOVES.to_vec()).unwrap();

    let game = Game::new(&game_state, &legal_moves).unwrap();
    let subject = game.all_available_moves().unwrap();

    assert_eq!(subject, HashSet::with_capacity(0));
}
