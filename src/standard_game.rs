use crate::game_move::GameMove;
use crate::legal_moves::LegalMoves;
use crate::position_data::PositionData;

lazy_static! {
    pub static ref STANDARD_MOVES: LegalMoves = LegalMoves(vec![
                // 00
                PositionData(hashset![
                    GameMove {
                        leapt_space: 01,
                        destination_space: 03,
                    },
                    GameMove {
                        leapt_space: 02,
                        destination_space: 05,
                    },
                ]),
                // 01
                PositionData(hashset![
                    GameMove {
                        leapt_space: 03,
                        destination_space: 06,
                    },
                    GameMove {
                        leapt_space: 04,
                        destination_space: 08,
                    },
                ]),
                // 02
                PositionData(hashset![
                    GameMove {
                        leapt_space: 04,
                        destination_space: 07,
                    },
                    GameMove {
                        leapt_space: 05,
                        destination_space: 09,
                    },
                ]),
                // 03
                PositionData(hashset![
                    GameMove {
                        leapt_space: 01,
                        destination_space: 00,
                    },
                    GameMove {
                        leapt_space: 06,
                        destination_space: 10,
                    },
                    GameMove {
                        leapt_space: 04,
                        destination_space: 05,
                    },
                    GameMove {
                        leapt_space: 07,
                        destination_space: 12,
                    },
                ]),
                // 04
                PositionData(hashset![
                    GameMove {
                        leapt_space: 07,
                        destination_space: 11,
                    },
                    GameMove {
                        leapt_space: 08,
                        destination_space: 13,
                    },
                ]),
                // 05
                PositionData(hashset![
                    GameMove {
                        leapt_space: 02,
                        destination_space: 00,
                    },
                    GameMove {
                        leapt_space: 04,
                        destination_space: 03,
                    },
                    GameMove {
                        leapt_space: 08,
                        destination_space: 12,
                    },
                    GameMove {
                        leapt_space: 09,
                        destination_space: 14,
                    },
                ]),
                // 06
                PositionData(hashset![
                    GameMove {
                        leapt_space: 03,
                        destination_space: 01,
                    },
                    GameMove {
                        leapt_space: 07,
                        destination_space: 08,
                    },
                ]),
                // 07
                PositionData(hashset![
                    GameMove {
                        leapt_space: 04,
                        destination_space: 02,
                    },
                    GameMove {
                        leapt_space: 08,
                        destination_space: 09,
                    },
                ]),
                // 08
                PositionData(hashset![
                    GameMove {
                        leapt_space: 04,
                        destination_space: 01,
                    },
                    GameMove {
                        leapt_space: 07,
                        destination_space: 06,
                    },
                ]),
                // 09
                PositionData(hashset![
                    GameMove {
                        leapt_space: 05,
                        destination_space: 02,
                    },
                    GameMove {
                        leapt_space: 08,
                        destination_space: 07,
                    },
                ]),
                // 10
                PositionData(hashset![
                    GameMove {
                        leapt_space: 06,
                        destination_space: 03,
                    },
                    GameMove {
                        leapt_space: 11,
                        destination_space: 12,
                    },
                ]),
                // 11
                PositionData(hashset![
                    GameMove {
                        leapt_space: 07,
                        destination_space: 04,
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
                        leapt_space: 07,
                        destination_space: 03,
                    },
                    GameMove {
                        leapt_space: 08,
                        destination_space: 05,
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
                        leapt_space: 08,
                        destination_space: 04,
                    },
                ]),
                // 14
                PositionData(hashset![
                    GameMove {
                        leapt_space: 13,
                        destination_space: 12,
                    },
                    GameMove {
                        leapt_space: 09,
                        destination_space: 05,
                    },
                ]),
        ]);
}
