use crate::game::game::Game;
use crate::game::game_move::GameMove;
use crate::GameError;
use std::collections::HashSet;

impl<'a> Game<'a> {
    pub fn solve(self) -> Result<HashSet<Vec<GameMove>>, GameError> {
        let mut solved: HashSet<Vec<GameMove>> = hashset![];
        let mut queue: Vec<(Game, Vec<GameMove>)> = vec![(self, vec![])];

        while !queue.is_empty() {
            let (current_game, previous_moves) = queue.pop().unwrap(); // `while` protects us
            for game_move in current_game.all_available_moves()? {
                let new_game = current_game.make_move(game_move)?;
                let mut updated_moves = previous_moves.clone();
                updated_moves.push(*game_move);

                if new_game.filled() == 1 {
                    solved.insert(updated_moves.clone());
                }

                queue.push((new_game, updated_moves));
            }
        }

        Ok(solved)
    }
}

// =================================================================================================
// Tests
// =================================================================================================
