use crate::game_move::GameMove;
use crate::legal_moves::LegalMoves;
use crate::position_data::PositionData;

lazy_static! {
    pub static ref STANDARD_MOVES: Vec<PositionData> = vec![
                // 0
                PositionData(hashset![
                    GameMove {
                        leapt_space: 1,
                        destination_space: 3,
                    },
                    GameMove {
                        leapt_space: 2,
                        destination_space: 5,
                    },
                ]),
                // 1
                PositionData(hashset![
                    GameMove {
                        leapt_space: 3,
                        destination_space: 6,
                    },
                    GameMove {
                        leapt_space: 4,
                        destination_space: 8,
                    },
                ]),
                // 2
                PositionData(hashset![
                    GameMove {
                        leapt_space: 4,
                        destination_space: 7,
                    },
                    GameMove {
                        leapt_space: 5,
                        destination_space: 9,
                    },
                ]),
                // 3
                PositionData(hashset![
                    GameMove {
                        leapt_space: 1,
                        destination_space: 0,
                    },
                    GameMove {
                        leapt_space: 6,
                        destination_space: 10,
                    },
                    GameMove {
                        leapt_space: 4,
                        destination_space: 5,
                    },
                    GameMove {
                        leapt_space: 7,
                        destination_space: 12,
                    },
                ]),
                // 4
                PositionData(hashset![
                    GameMove {
                        leapt_space: 7,
                        destination_space: 11,
                    },
                    GameMove {
                        leapt_space: 8,
                        destination_space: 13,
                    },
                ]),
                // 5
                PositionData(hashset![
                    GameMove {
                        leapt_space: 2,
                        destination_space: 0,
                    },
                    GameMove {
                        leapt_space: 4,
                        destination_space: 3,
                    },
                    GameMove {
                        leapt_space: 8,
                        destination_space: 12,
                    },
                    GameMove {
                        leapt_space: 9,
                        destination_space: 14,
                    },
                ]),
                // 6
                PositionData(hashset![
                    GameMove {
                        leapt_space: 3,
                        destination_space: 1,
                    },
                    GameMove {
                        leapt_space: 7,
                        destination_space: 8,
                    },
                ]),
                // 7
                PositionData(hashset![
                    GameMove {
                        leapt_space: 4,
                        destination_space: 2,
                    },
                    GameMove {
                        leapt_space: 8,
                        destination_space: 9,
                    },
                ]),
                // 8
                PositionData(hashset![
                    GameMove {
                        leapt_space: 4,
                        destination_space: 1,
                    },
                    GameMove {
                        leapt_space: 7,
                        destination_space: 6,
                    },
                ]),
                // 9
                PositionData(hashset![
                    GameMove {
                        leapt_space: 5,
                        destination_space: 2,
                    },
                    GameMove {
                        leapt_space: 8,
                        destination_space: 7,
                    },
                ]),
                // 10
                PositionData(hashset![
                    GameMove {
                        leapt_space: 6,
                        destination_space: 3,
                    },
                    GameMove {
                        leapt_space: 11,
                        destination_space: 12,
                    },
                ]),
                // 11
                PositionData(hashset![
                    GameMove {
                        leapt_space: 7,
                        destination_space: 4,
                    },
                    GameMove {
                        leapt_space: 12,
                        destination_space: 13,
                    },
                ]),
                // 12
                PositionData(hashset![
                    GameMove {
                        leapt_space: 11,
                        destination_space: 10,
                    },
                    GameMove {
                        leapt_space: 7,
                        destination_space: 3,
                    },
                    GameMove {
                        leapt_space: 8,
                        destination_space: 5,
                    },
                    GameMove {
                        leapt_space: 13,
                        destination_space: 14,
                    },
                ]),
                // 13
                PositionData(hashset![
                    GameMove {
                        leapt_space: 12,
                        destination_space: 11,
                    },
                    GameMove {
                        leapt_space: 8,
                        destination_space: 4,
                    },
                ]),
                // 14
                PositionData(hashset![
                    GameMove {
                        leapt_space: 13,
                        destination_space: 12,
                    },
                    GameMove {
                        leapt_space: 9,
                        destination_space: 5,
                    },
                ]),
        ];
}
