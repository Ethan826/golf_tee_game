use clap::{App, Arg, SubCommand};
use golf_tee_game::game::game::Game;
use golf_tee_game::game::standard_game::build_standard_game_args;

static EMPTY_SPACE: &str = "empty space";

fn main() {
    let mut app = App::new("Golf Tee Game Solver")
        .version("0.1")
        .author("Ethan Kent")
        .about(
            "Solves the Golf Tee Game (the game on Cracker Barrel tables).\n\nUses the following \
             numbering system for positions on the game board.\n\n        00\n      01  02\n    \
             03  04  05\n  06  07  08  09\n10  11  12  13  14",
        )
        .arg(
            Arg::with_name(EMPTY_SPACE)
                .help("Sets the empty space(s) at the beginning of the game")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .subcommand(SubCommand::with_name("solutions").about("Print solutions to the game"))
        .subcommand(SubCommand::with_name("count").about("Print the number of unique solutions"));

    let matches = app.clone().get_matches(); // FIXME
    let empty_position = matches
        .value_of(EMPTY_SPACE)
        .unwrap()
        .parse::<usize>()
        .expect("Invalid empty space. You must supply a numeral.");

    match matches.subcommand_name() {
        Some("solutions") => solve_for_given_position(empty_position),
        Some("count") => print_total_moves_from_position(empty_position),
        _ => {
            app.print_long_help().unwrap();
            println!();
        }
    }
}

#[allow(dead_code)]
fn print_total_moves_from_position(pos: usize) {
    let (game_state, legal_moves) = build_standard_game_args(pos);
    let game = Game::new(game_state, &legal_moves).unwrap();

    let solutions = game.solve().unwrap();
    println!(
        "Starting from position {}, there are {} solutions.",
        pos,
        solutions.len()
    );
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
