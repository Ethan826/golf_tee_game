use crate::game_move::GameMove;
use std::collections::HashSet;

/// Represents the legal moves from a given position on the game board.
/// For example, given the following game board
/// ```text
///   0
///  1 2
/// 3 4 5
/// ```
/// the legal moves from position 0 are
/// ```
/// # use golf_tee_game::game_move::GameMove;
/// GameMove {
///     leapt_space: 1,
///     destination_space: 3,
/// };
/// ```
/// and
/// ```
/// # use golf_tee_game::game_move::GameMove;
/// GameMove {
///     leapt_space: 2,
///     destination_space: 5,
/// };
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionData(pub HashSet<GameMove>);
