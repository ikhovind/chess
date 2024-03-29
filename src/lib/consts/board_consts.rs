#![allow(unused)]
pub const FILE_MASKS: [u64; 8] =
    [
        72340172838076673,
        144680345676153346,
        289360691352306692,
        578721382704613384,
        1157442765409226768,
        2314885530818453536,
        4629771061636907072,
        9259542123273814144,
    ];

pub const RANK_MASKS: [u64; 8] =
    [
        255,
        65280,
        16711680,
        4278190080,
        1095216660480,
        280375465082880,
        71776119061217280,
        18374686479671623680,
    ];
pub static CENTRE: u64 = 103481868288;
pub static EXTENDED_CENTRE: u64 = 66229406269440;

pub const DIAGONAL_MASKS: [u64; 15] = /*from top eft to bottom right*/
    [
        0x1, 0x102, 0x10204, 0x1020408, 0x102040810, 0x10204081020, 0x1020408102040,
        0x102040810204080, 0x204081020408000, 0x408102040800000, 0x810204080000000,
        0x1020408000000000, 0x2040800000000000, 0x4080000000000000, 0x8000000000000000
    ];

/*from top right to bottom eft*/
pub const ANTI_DIAGONAL_MASKS: [u64; 15] =
    [
        0x80, 0x8040, 0x804020, 0x80402010, 0x8040201008, 0x804020100804, 0x80402010080402,
        0x8040201008040201, 0x4020100804020100, 0x2010080402010000, 0x1008040201000000,
        0x804020100000000, 0x402010000000000, 0x201000000000000, 0x100000000000000
    ];
pub const WHITE_LONG_ORG_ROOK: u64 = 1 << 0;
pub const WHITE_SHORT_ORG_ROOK: u64 = 1 << 7;
pub const BLACK_LONG_ORG_ROOK: u64 = 1 << 56;
pub const BLACK_SHORT_ORG_ROOK: u64 = 1 << 63;

pub const WHITE_SHORT_CASTLE_KING: u64 = 1 << 6;
pub const WHITE_SHORT_CASTLE_ROOK: u64 = 1 << 5;
pub const WHITE_LONG_CASTLE_KING: u64 = 1 << 2;
pub const WHITE_LONG_CASTLE_ROOK: u64 = 1 << 3;

pub const BLACK_SHORT_CASTLE_KING: u64 = 1 << 62;
pub const BLACK_SHORT_CASTLE_ROOK: u64 = 1 << 61;
pub const BLACK_LONG_CASTLE_KING: u64 = 1 << 60;
pub const BLACK_LONG_CASTLE_ROOK: u64 = 1 << 59;
pub const BLACK_KING: u64 = 1 << 60;
pub const WHITE_KING: u64 = 1 << 4;

pub const TO_MASK: u8 = 0b00110000;
pub const FROM_MASK: u8 = 0b11000000;
pub const TYPE_MASK: u8 = 0b11000000;
pub const MOVE_MASK: u8 = 0b00111111;

pub const NORMAL_MOVE: u8 = 0;
pub const DOUBLE_PAWN: u8 = 0b00010000;
pub const TAKES: u8 = 0b00100000;
pub const EN_PASSANT: u8 = 0b00110000; // - EP

pub static QUEEN: u8 = 0;
pub static ROOK: u8 = 1;
pub static BISHOP: u8 = 2;
pub static KNIGHT: u8 = 3;

pub const PROM_Q: u8 = 0b01000000;
// - Queen
pub const PROM_R: u8 = 0b01010000;
// - Rook
pub const PROM_B: u8 = 0b01100000;
// - Bishop
pub const PROM_N: u8 = 0b01110000; // - Knight

pub const TAKE_PROM_Q: u8 = 0b10000000;
// - TAKES into Queen
pub const TAKE_PROM_R: u8 = 0b10010000;
// - TAKES into Rook
pub const TAKE_PROM_B: u8 = 0b10100000;
// - TAKES into Bishop
pub const TAKE_PROM_N: u8 = 0b10110000; // - TAKES into Knight

pub const SHORT_CASTLE: u8 = 0b11000000;
// - short castle
pub const LONG_CASTLE: u8 = 0b11010000; // - long castle

pub const P_INDEX: usize = 0;
pub const N_INDEX: usize = 2;
pub const B_INDEX: usize = 4;
pub const R_INDEX: usize = 6;
pub const Q_INDEX: usize = 8;
pub const K_INDEX: usize = 10;

pub const P_VAL_INDEX: usize = 0;
pub const N_VAL_INDEX: usize = 1;
pub const B_VAL_INDEX: usize = 2;
pub const R_VAL_INDEX: usize = 3;
pub const Q_VAL_INDEX: usize = 4;
pub const K_VAL_INDEX: usize = 5;

// todo tweak king value
pub const PIECE_VALUES: [i16; 6] = [100, 300, 300, 500, 900, 0];
pub const P_INF: i16 = 20000;
pub const N_INF: i16 = -20000;
pub const N_MATE: i16 = -10000;
pub const P_MATE: i16 = 10000;

