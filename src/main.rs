use golf_tee_game::game::game::Game;
use golf_tee_game::game::standard_game::build_standard_game_args;

fn main() {
    // print_total_moves_from_each_position();
    solve_for_given_position(0);
}

#[allow(dead_code)]
fn print_total_moves_from_each_position() {
    (0..15).for_each(|i| {
        let (game_state, legal_moves) = build_standard_game_args(i);
        let game = Game::new(game_state, &legal_moves).unwrap();

        let solutions = game.solve().unwrap();
        println!("Starting from position {}, it's {}", i, solutions.len());
    });
}

#[allow(dead_code)]
fn solve_for_given_position(pos: usize) {
    let (game_state, legal_moves) = build_standard_game_args(pos);
    let game = Game::new(game_state, &legal_moves).unwrap();

    let solutions = game.solve().unwrap();

    for (i, solution) in solutions.iter().enumerate() {
        println!(
            "\nBegin solution {} of {}\n=============================\n",
            i + 1,
            solutions.len()
        );
        for step in solution {
            println!("{}", step);
        }
    }
}
