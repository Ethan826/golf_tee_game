use crate::game::game::Game;
use crate::game::game_move::GameMove;
use crate::game::legal_moves::LegalMoves;
use std::collections::HashSet;

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_foo() {
    use crate::game::standard_game::build_standard_game_args;
    let (game_state, legal_moves) = build_standard_game_args(4);
    let game = Game::new(game_state, &legal_moves).unwrap();

    let mut frontier = vec![game];
    let candidate_games: Vec<GameMove> = vec![];

    while !frontier.is_empty() {
        let mut next: Vec<Game> = Vec::new();
        for game in &frontier {
            for game_move in game.all_available_moves().unwrap() {
                println!("Making move {:?}", game_move);
                let new_game = game.make_move(game_move).unwrap();
                println!("{:#?}", new_game.filled());
                next.push(new_game);
            }
        }
        frontier = next;
    }

    assert!(false);
}
