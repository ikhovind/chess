use std::cmp::Ordering;
use std::fmt;
use num_format::Locale::gu;
use crate::Board;

use crate::consts::board_consts::*;
use crate::move_gen::pieces;

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    /*
    pub const NORMAL_MOVE: u8 = 0;
    pub const DOUBLE_PAWN: u8 = 0b00010000;
    pub const TAKES: u8       = 0b00100000;
    pub const EN_PASSANT: u8 =  0b00110000; // - EP

    pub const PROM_Q: u8      = 0b01000000; // - Queen
    pub const PROM_R: u8      = 0b01010000; // - Rook
    pub const PROM_B: u8      = 0b01100000; // - Bishop
    pub const PROM_N: u8      = 0b01110000; // - Knight

    pub const TAKE_PROM_Q: u8 = 0b10000000; // - TAKES into Queen
    pub const TAKE_PROM_R: u8 = 0b10010000; // - TAKES into Rook
    pub const TAKE_PROM_B: u8 = 0b10100000; // - TAKES into Bishop
    pub const TAKE_PROM_N: u8 = 0b10110000; // - TAKES into Knight

    pub const SHORT_CASTLE: u8= 0b11000000; // - short castle
    pub const LONG_CASTLE: u8 = 0b11010000; // - long castle
    */
}

pub static BASIS: u8 = 0b00111111;


impl Move {
    pub fn parse_move(notation: &str, b: &Board) -> Result<Move, String>{
        if notation.len() == 4 {

            let from_column = notation.as_bytes()[0] - 97;
            let from_row = (notation.as_bytes()[1] - 49);
            let from_ix = from_column + from_row * 8;

            let to_column = notation.as_bytes()[2] - 97;
            let to_row = (notation.as_bytes()[3] - 49);
            let to_ix = to_column + to_row * 8;

            if (b.pieces[P_INDEX] | b.pieces[P_INDEX + 1]) & (1 << from_ix) != 0
                && (1 << to_ix) & (RANK_MASKS[3] | RANK_MASKS[4]) != 0 {
                return Ok(Move::new_double_push(from_ix, to_ix));
            }
            if  (b.pieces[P_INDEX] | b.pieces[P_INDEX + 1]) & (1 << from_ix) != 0
                && (1 << to_ix) & (RANK_MASKS[0] | RANK_MASKS[7]) != 0 {
                return Ok(Move::new_promotion(from_ix, to_ix, (b.get_white_pieces() | b.get_black_pieces()) & (1 << to_ix) != 0, QUEEN));
            }
            if  (b.pieces[P_INDEX] | b.pieces[P_INDEX + 1]) & (1 << from_ix) != 0
                && b.get_empty() & (1 << to_ix) != 0
                && to_column != from_column {
                return Ok(Move::new_ep(from_ix, to_ix));
            }
            if  (b.pieces[K_INDEX] | b.pieces[K_INDEX + 1]) & (1 << from_ix) & (WHITE_KING | BLACK_KING) != 0
                && (1 << to_ix) & (WHITE_LONG_CASTLE_ROOK | WHITE_SHORT_CASTLE_KING | BLACK_LONG_CASTLE_KING | BLACK_SHORT_CASTLE_KING) != 0 {
                return Ok(Move::new_castle(from_ix, to_ix));
            }
            return Ok(Move::new_move(from_ix, to_ix, (b.get_white_pieces() | b.get_black_pieces()) & (1 << to_ix) != 0));
        }
        return Err("Not a legal move".to_string());
    }

    pub fn new_move(_from: u8, _to: u8, is_capture: bool) -> Move {
        return Move {
            from: (_from & BASIS)
                | (if is_capture { TAKES & FROM_MASK } else { 0 }),
            to: (_to & BASIS)
                | (if is_capture { TAKES << 2 } else { 0 }),
        };
    }

    pub fn new_double_push(_from: u8, _to: u8) -> Move {
        let mv = Move {
            from: (_from & BASIS) | (DOUBLE_PAWN & FROM_MASK),
            to: (_to & BASIS) | ((DOUBLE_PAWN & TO_MASK) << 2),
        };
        return mv;
    }

    pub fn new_promotion(_from: u8, _to: u8, is_capture: bool, promote_to: u8) -> Move {
        let typ;
        if !is_capture {
            // todo error handling
            match promote_to {
                0 => typ = PROM_Q,
                1 => typ = PROM_R,
                2 => typ = PROM_B,
                3 => typ = PROM_N,
                _ => typ = PROM_Q
            }
        } else {
            // todo error handling
            match promote_to {
                0 => typ = TAKE_PROM_Q,
                1 => typ = TAKE_PROM_R,
                2 => typ = TAKE_PROM_B,
                3 => typ = TAKE_PROM_N,
                _ => typ = TAKE_PROM_Q
            }
        }
        return Move {
            from: (_from & BASIS) | (typ & FROM_MASK),
            to: (_to & BASIS) | (typ << 2),
        };
    }

    pub fn new_ep(_from: u8, _to: u8) -> Move {
        let m = Move {
            from: (_from & BASIS) | (EN_PASSANT & FROM_MASK),
            to: (_to & BASIS) | (EN_PASSANT << 2),
        };
        return m;
    }

    pub fn new_castle(_from: u8, _to: u8) -> Move {
        let typ = if _to == 62 || _to == 6 {
            SHORT_CASTLE
        } else {
            LONG_CASTLE
        };
        return Move {
            from: (_from & BASIS) | (typ & FROM_MASK),
            to: (_to & BASIS) | ((typ & TO_MASK) << 2),
        };
    }

    pub fn last_move_was_double_push(m: Move) -> bool {
        return (((m.to & !MOVE_MASK) >> 2) | (m.from & !MOVE_MASK)) == DOUBLE_PAWN;
    }

    pub fn is_capture(&self) -> bool {
        let m_type =  (((self.to & !MOVE_MASK) >> 2) | (self.from & !MOVE_MASK));
        return m_type == TAKES ||
            m_type == EN_PASSANT ||
            m_type == TAKE_PROM_B ||
            m_type == TAKE_PROM_Q ||
            m_type == TAKE_PROM_N ||
            m_type == TAKE_PROM_R;
    }

    pub fn is_promotion(&self) -> bool {
        let m_type = self.get_mv_type();
        return
            m_type == PROM_B ||
            m_type == PROM_Q ||
            m_type == PROM_N ||
            m_type == PROM_R ||
            m_type == TAKE_PROM_B ||
            m_type == TAKE_PROM_Q ||
            m_type == TAKE_PROM_N ||
            m_type == TAKE_PROM_R;
    }

    pub fn get_from_sq(&self) -> u8 {
        return self.from & MOVE_MASK;
    }

    pub fn get_to_sq(&self) -> u8 {
        return self.to & MOVE_MASK;
    }

    pub fn get_mv_type(&self) -> u8 {
        return ((self.to & !MOVE_MASK) >> 2) | (self.from & !MOVE_MASK);
    }

    pub fn get_promotion_value(&self) -> i16 {
        if !self.is_promotion() {
            return 0;
        }
        let mv_type = self.get_mv_type();
        if mv_type == PROM_Q || mv_type == TAKE_PROM_Q {
            return PIECE_VALUES[Q_VAL_INDEX];
        }
        else if mv_type == PROM_N || mv_type == TAKE_PROM_N {
            return PIECE_VALUES[N_VAL_INDEX];
        }
        else if mv_type == PROM_R || mv_type == TAKE_PROM_R {
            return PIECE_VALUES[R_VAL_INDEX];
        }
        else if mv_type == PROM_B || mv_type == TAKE_PROM_B {
            return PIECE_VALUES[B_VAL_INDEX];
        }
        else { return 0 }
    }

    pub fn guess_score(&self, watched_by_p: u64, b: &Board) -> i16 {
        let mut guess = 0;
        let mut mv_piece_value = 0;
        let mut cap_piece_value = -1;
        for (ix, it) in b.pieces.iter().enumerate() {
            mv_piece_value = ((*it & (1u64 << self.get_from_sq())) as i16 * i16::MAX) & PIECE_VALUES[ix / 2];
            cap_piece_value = ((*it & (1u64 << self.get_to_sq())) as i16 * i16::MAX) & PIECE_VALUES[ix / 2];
        }
        if cap_piece_value != -1 {
            guess = 10 * cap_piece_value - mv_piece_value;
        }

        if self.is_promotion() {
            guess += self.get_promotion_value();
        }
        if watched_by_p & (1u64 << self.get_to_sq()) != 0 {
            guess -= mv_piece_value;
        }
        return guess;
    }
}






impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match (self.from & MOVE_MASK) % 8 {
            0 => { fmt.write_str("a")?; }
            1 => { fmt.write_str("b")?; }
            2 => { fmt.write_str("c")?; }
            3 => { fmt.write_str("d")?; }
            4 => { fmt.write_str("e")?; }
            5 => { fmt.write_str("f")?; }
            6 => { fmt.write_str("g")?; }
            7 => { fmt.write_str("h")?; }
            _ => { fmt.write_str("error")?; }
        }
        match (self.from & MOVE_MASK) / 8 {
            0 => { fmt.write_str("1")?; }
            1 => { fmt.write_str("2")?; }
            2 => { fmt.write_str("3")?; }
            3 => { fmt.write_str("4")?; }
            4 => { fmt.write_str("5")?; }
            5 => { fmt.write_str("6")?; }
            6 => { fmt.write_str("7")?; }
            7 => { fmt.write_str("8")?; }
            _ => { fmt.write_str("error")?; }
        }
        match (self.to & MOVE_MASK) % 8 {
            0 => { fmt.write_str("a")?; }
            1 => { fmt.write_str("b")?; }
            2 => { fmt.write_str("c")?; }
            3 => { fmt.write_str("d")?; }
            4 => { fmt.write_str("e")?; }
            5 => { fmt.write_str("f")?; }
            6 => { fmt.write_str("g")?; }
            7 => { fmt.write_str("h")?; }
            _ => { fmt.write_str("error")?; }
        }
        match (self.to & MOVE_MASK) / 8 {
            0 => { fmt.write_str("1")?; }
            1 => { fmt.write_str("2")?; }
            2 => { fmt.write_str("3")?; }
            3 => { fmt.write_str("4")?; }
            4 => { fmt.write_str("5")?; }
            5 => { fmt.write_str("6")?; }
            6 => { fmt.write_str("7")?; }
            7 => { fmt.write_str("8")?; }
            _ => { fmt.write_str("error")?; }
        }
        match self.from & TYPE_MASK | ((self.to & TYPE_MASK) >> 2) {
            PROM_Q => {
                fmt.write_str("q").expect("ERR");
            }
            PROM_R => { fmt.write_str("r").expect("ERR"); }
            PROM_B => { fmt.write_str("b").expect("ERR"); }
            PROM_N => { fmt.write_str("n").expect("ERR"); }
            TAKE_PROM_Q => {
                fmt.write_str("q").expect("ERR");
            }
            TAKE_PROM_R => { fmt.write_str("r").expect("ERR"); }
            TAKE_PROM_B => { fmt.write_str("b").expect("ERR"); }
            TAKE_PROM_N => { fmt.write_str("n").expect("ERR"); }
            _ => {}
        }
        Ok(())
    }
}


impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
            && self.to == other.to
    }
}
impl Eq for Move {}

impl PartialOrd<Self> for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(Ordering::Less);
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.from.cmp(&other.from);
    }
}
