use super::game_move::GameMove;
use super::legal_moves::LegalMoves;
use super::position_data::PositionData;

lazy_static! {
    pub static ref STANDARD_MOVES: Vec<PositionData> = vec![
                // 0
                PositionData(hashset![
                    GameMove {
                        starting_space: 0,
                        leapt_space: 1,
                        destination_space: 3,
                    },
                    GameMove {
                        starting_space: 0,
                        leapt_space: 2,
                        destination_space: 5,
                    },
                ]),
                // 1
                PositionData(hashset![
                    GameMove {
                        starting_space: 1,
                        leapt_space: 3,
                        destination_space: 6,
                    },
                    GameMove {
                        starting_space: 1,
                        leapt_space: 4,
                        destination_space: 8,
                    },
                ]),
                // 2
                PositionData(hashset![
                    GameMove {
                        starting_space: 2,
                        leapt_space: 4,
                        destination_space: 7,
                    },
                    GameMove {
                        starting_space: 2,
                        leapt_space: 5,
                        destination_space: 9,
                    },
                ]),
                // 3
                PositionData(hashset![
                    GameMove {
                        starting_space: 3,
                        leapt_space: 1,
                        destination_space: 0,
                    },
                    GameMove {
                        starting_space: 3,
                        leapt_space: 6,
                        destination_space: 10,
                    },
                    GameMove {
                        starting_space: 3,
                        leapt_space: 4,
                        destination_space: 5,
                    },
                    GameMove {
                        starting_space: 3,
                        leapt_space: 7,
                        destination_space: 12,
                    },
                ]),
                // 4
                PositionData(hashset![
                    GameMove {
                        starting_space: 4,
                        leapt_space: 7,
                        destination_space: 11,
                    },
                    GameMove {
                        starting_space: 4,
                        leapt_space: 8,
                        destination_space: 13,
                    },
                ]),
                // 5
                PositionData(hashset![
                    GameMove {
                        starting_space: 5,
                        leapt_space: 2,
                        destination_space: 0,
                    },
                    GameMove {
                        starting_space: 5,
                        leapt_space: 4,
                        destination_space: 3,
                    },
                    GameMove {
                        starting_space: 5,
                        leapt_space: 8,
                        destination_space: 12,
                    },
                    GameMove {
                        starting_space: 5,
                        leapt_space: 9,
                        destination_space: 14,
                    },
                ]),
                // 6
                PositionData(hashset![
                    GameMove {
                        starting_space: 6,
                        leapt_space: 3,
                        destination_space: 1,
                    },
                    GameMove {
                        starting_space: 6,
                        leapt_space: 7,
                        destination_space: 8,
                    },
                ]),
                // 7
                PositionData(hashset![
                    GameMove {
                        starting_space: 7,
                        leapt_space: 4,
                        destination_space: 2,
                    },
                    GameMove {
                        starting_space: 7,
                        leapt_space: 8,
                        destination_space: 9,
                    },
                ]),
                // 8
                PositionData(hashset![
                    GameMove {
                        starting_space: 8,
                        leapt_space: 4,
                        destination_space: 1,
                    },
                    GameMove {
                        starting_space: 8,
                        leapt_space: 7,
                        destination_space: 6,
                    },
                ]),
                // 9
                PositionData(hashset![
                    GameMove {
                        starting_space: 9,
                        leapt_space: 5,
                        destination_space: 2,
                    },
                    GameMove {
                        starting_space: 9,
                        leapt_space: 8,
                        destination_space: 7,
                    },
                ]),
                // 10
                PositionData(hashset![
                    GameMove {
                        starting_space: 10,
                        leapt_space: 6,
                        destination_space: 3,
                    },
                    GameMove {
                        starting_space: 10,
                        leapt_space: 11,
                        destination_space: 12,
                    },
                ]),
                // 11
                PositionData(hashset![
                    GameMove {
                        starting_space: 11,
                        leapt_space: 7,
                        destination_space: 4,
                    },
                    GameMove {
                        starting_space: 11,
                        leapt_space: 12,
                        destination_space: 13,
                    },
                ]),
                // 12
                PositionData(hashset![
                    GameMove {
                        starting_space: 12,
                        leapt_space: 11,
                        destination_space: 10,
                    },
                    GameMove {
                        starting_space: 12,
                        leapt_space: 7,
                        destination_space: 3,
                    },
                    GameMove {
                        starting_space: 12,
                        leapt_space: 8,
                        destination_space: 5,
                    },
                    GameMove {
                        starting_space: 12,
                        leapt_space: 13,
                        destination_space: 14,
                    },
                ]),
                // 13
                PositionData(hashset![
                    GameMove {
                        starting_space: 13,
                        leapt_space: 12,
                        destination_space: 11,
                    },
                    GameMove {
                        starting_space: 13,
                        leapt_space: 8,
                        destination_space: 4,
                    },
                ]),
                // 14
                PositionData(hashset![
                    GameMove {
                        starting_space: 14,
                        leapt_space: 13,
                        destination_space: 12,
                    },
                    GameMove {
                        starting_space: 14,
                        leapt_space: 9,
                        destination_space: 5,
                    },
                ]),
        ];
}
