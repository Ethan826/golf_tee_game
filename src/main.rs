use std::collections::HashSet;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Move {
    leapt_space: u8,
    destination_space: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct Position(HashSet<Move>);

lazy_static! {
    static ref MOVES: Vec<Position> = vec![
                // 00
                Position(hashset![
                    Move {
                        leapt_space: 01,
                        destination_space: 03,
                    },
                    Move {
                        leapt_space: 02,
                        destination_space: 05,
                    },
                ]),
                // 01
                Position(hashset![
                    Move {
                        leapt_space: 03,
                        destination_space: 06,
                    },
                    Move {
                        leapt_space: 04,
                        destination_space: 08,
                    },
                ]),
                // 02
                Position(hashset![
                    Move {
                        leapt_space: 04,
                        destination_space: 07,
                    },
                    Move {
                        leapt_space: 05,
                        destination_space: 09,
                    },
                ]),
                // 03
                Position(hashset![
                    Move {
                        leapt_space: 01,
                        destination_space: 00,
                    },
                    Move {
                        leapt_space: 06,
                        destination_space: 10,
                    },
                    Move {
                        leapt_space: 04,
                        destination_space: 05,
                    },
                    Move {
                        leapt_space: 07,
                        destination_space: 12,
                    },
                ]),
                // 04
                Position(hashset![
                    Move {
                        leapt_space: 07,
                        destination_space: 11,
                    },
                    Move {
                        leapt_space: 08,
                        destination_space: 13,
                    },
                ]),
                // 05
                Position(hashset![
                    Move {
                        leapt_space: 02,
                        destination_space: 00,
                    },
                    Move {
                        leapt_space: 04,
                        destination_space: 03,
                    },
                    Move {
                        leapt_space: 08,
                        destination_space: 12,
                    },
                    Move {
                        leapt_space: 09,
                        destination_space: 14,
                    },
                ]),
                // 06
                Position(hashset![
                    Move {
                        leapt_space: 03,
                        destination_space: 01,
                    },
                    Move {
                        leapt_space: 07,
                        destination_space: 08,
                    },
                ]),
                // 07
                Position(hashset![
                    Move {
                        leapt_space: 04,
                        destination_space: 02,
                    },
                    Move {
                        leapt_space: 08,
                        destination_space: 09,
                    },
                ]),
                // 08
                Position(hashset![
                    Move {
                        leapt_space: 04,
                        destination_space: 01,
                    },
                    Move {
                        leapt_space: 07,
                        destination_space: 06,
                    },
                ]),
                // 09
                Position(hashset![
                    Move {
                        leapt_space: 05,
                        destination_space: 02,
                    },
                    Move {
                        leapt_space: 08,
                        destination_space: 07,
                    },
                ]),
                // 10
                Position(hashset![
                    Move {
                        leapt_space: 06,
                        destination_space: 03,
                    },
                    Move {
                        leapt_space: 11,
                        destination_space: 12,
                    },
                ]),
                // 11
                Position(hashset![
                    Move {
                        leapt_space: 07,
                        destination_space: 04,
                    },
                    Move {
                        leapt_space: 12,
                        destination_space: 13,
                    },
                ]),
                // 12
                Position(hashset![
                    Move {
                        leapt_space: 11,
                        destination_space: 10,
                    },
                    Move {
                        leapt_space: 07,
                        destination_space: 03,
                    },
                    Move {
                        leapt_space: 08,
                        destination_space: 05,
                    },
                    Move {
                        leapt_space: 13,
                        destination_space: 14,
                    },
                ]),
                // 13
                Position(hashset![
                    Move {
                        leapt_space: 12,
                        destination_space: 11,
                    },
                    Move {
                        leapt_space: 08,
                        destination_space: 04,
                    },
                ]),
                // 14
                Position(hashset![
                    Move {
                        leapt_space: 13,
                        destination_space: 12,
                    },
                    Move {
                        leapt_space: 09,
                        destination_space: 05,
                    },
                ]),
        ];
}

fn main() {
    println!("{:#?}", *MOVES);
}
