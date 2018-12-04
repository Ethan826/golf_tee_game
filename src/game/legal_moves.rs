use super::position_data::PositionData;
use crate::{is_triangular, GameError};

/// Represents all the legal moves for a particular game.
#[derive(Debug, PartialEq, Eq)]
pub struct LegalMoves(Vec<PositionData>);

impl LegalMoves {
    /// Given a `Vec` of `PositionData`, return an instance of `LegalMoves`.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of `input` is not a triangular number.
    pub fn new(input: Vec<PositionData>) -> Result<Self, GameError> {
        if is_triangular(input.len()) {
            Ok(LegalMoves(input))
        } else {
            Err(GameError::InvalidGameSize)?
        }
    }

    /// Retrieves the position data at a specified position.
    ///
    /// # Errors
    ///
    /// Returns an error if the position specified is not valid.
    pub fn position_data<'a>(&'a self, position: usize) -> Result<&'a PositionData, GameError> {
        match self.0.get(position) {
            Some(position_data) => Ok(position_data),
            None => Err(GameError::InvalidPosition),
        }
    }

    /// Return the length of legal moves.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_legal_moves_new_valid_size() {
    assert!(LegalMoves::new(vec![
        PositionData(hashset! {}),
        PositionData(hashset! {}),
        PositionData(hashset! {}),
    ])
    .is_ok());
}

#[test]
fn test_legal_moves_new_invalid_size() {
    let subject = LegalMoves::new(vec![PositionData(hashset! {}), PositionData(hashset! {})]);
    assert_eq!(subject.unwrap_err(), GameError::InvalidGameSize);
}

#[test]
fn test_legal_moves_position_data_valid() {
    use super::standard_game::STANDARD_MOVES;
    let subject = LegalMoves::new(STANDARD_MOVES.to_vec()).unwrap();
    assert!(subject.position_data(1).is_ok());
}

#[test]
fn test_legal_moves_position_data_invalid() {
    use super::standard_game::STANDARD_MOVES;
    let subject = LegalMoves::new(STANDARD_MOVES.to_vec()).unwrap();
    assert!(subject.position_data(19).is_err());
}
