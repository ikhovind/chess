use crate::opponent::game_stage::GameStage;
use crate::opponent::game_stage::GameStage::{EARLY, LATE, MIDDLE};

pub const EG_PAWN_TABLE: [i16; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

pub const MG_KNIGHT_TABLE: [i16; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub const EG_KNIGHT_TABLE: [i16; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub const MG_BISHOP_TABLE: [i16; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];


pub const EG_ROOK_TABLE: [i16; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];


pub const EG_QUEEN_TABLE: [i16; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

pub const MG_KING_TABLE: [i16; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

pub const EG_KING_TABLE: [i16; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];

const eg_tables: [[i16; 64]; 6] = [
    EG_PAWN_TABLE,
    EG_KNIGHT_TABLE,
    MG_BISHOP_TABLE,
    EG_ROOK_TABLE,
    EG_QUEEN_TABLE,
    EG_KING_TABLE
];

const mg_tables: [[i16; 64]; 6] = [
    EG_PAWN_TABLE,
    MG_KNIGHT_TABLE,
    MG_BISHOP_TABLE,
    EG_ROOK_TABLE,
    EG_QUEEN_TABLE,
    EG_KING_TABLE
];

const lg_tables: [[i16; 64]; 6] = [
    EG_PAWN_TABLE,
    MG_KNIGHT_TABLE,
    MG_BISHOP_TABLE,
    EG_ROOK_TABLE,
    EG_QUEEN_TABLE,
    MG_KING_TABLE
];

pub const eg_table: [[i16; 64]; 12] = calculate_board(EARLY);

pub const mg_table: [[i16; 64]; 12] = calculate_board(MIDDLE);

pub const lg_table: [[i16; 64]; 12] = calculate_board(LATE);

const fn calculate_board(stage: GameStage) -> [[i16; 64]; 12] {
    let table = match stage {
        GameStage::EARLY => {
            eg_tables
        }
        GameStage::MIDDLE => {
            mg_tables
        }
        GameStage::LATE => {
            lg_tables
        }
    };
    let mut ans: [[i16; 64]; 12] = [[0; 64]; 12];

    let mut i = 0;
    while i < 12 {
        let mut j = 0;
        while j < 64 {
            if i % 2 == 0 {
                ans[i][j] = table[i / 2][j];
            }
            else {
                ans[i][j] = table[i / 2][j ^ 56];
            }
            j += 1;
        }
        i += 1;
    }
    return ans;
}

pub fn eval_sq(square: usize, piece_type: usize, stage: GameStage) -> i16{
    return match stage {
        EARLY => {
            eg_tables[piece_type][square]
        }
        MIDDLE => {
            mg_tables[piece_type][square]
        }
        LATE => {
            lg_tables[piece_type][square]
        }
    };
}