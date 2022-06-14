pub use crate::game::board_consts::{FILE_MASKS, DIAGONAL_MASKS, ANTI_DIAGONAL_MASKS};
use crate::{mv, print_u64_bitboard};
use crate::game::board_consts::RANK_MASKS;
use crate::mv::{BISHOP, KNIGHT, Move, QUEEN, ROOK};
use crate::pieces::*;
use crate::pieces::bishop;
pub(crate) mod board_consts;

//[black, white]
pub struct Board {
    pub(crate) pawns: [u64; 2],
    pub(crate) knights: [u64; 2],
    pub(crate) bishops: [u64; 2],
    pub(crate) rooks: [u64; 2],
    pub(crate) queens: [u64; 2],
    pub(crate) kings: [u64; 2],
    pub(crate) black_pieces: u64,
    pub(crate) white_pieces: u64,
    pub(crate) empty: u64,
    pub(crate) white_long_c: bool,
    pub(crate) white_short_c: bool,
    pub(crate) black_long_c: bool,
    pub(crate) black_short_c: bool,
    pub(crate) watched_squares_white: u64,
    pub(crate) watched_squares_black: u64,
}

impl Board {
    pub fn from_fen(fen: String) -> Board {
        let mut _pawns = [0,0];
        let mut _bishops = [0,0];
        let mut _rooks = [0,0];
        let mut _knights = [0,0];
        let mut _queens = [0,0];
        let mut _kings = [0,0];

        let mut column : u32 = 0;
        let mut row = 7;
        let mut res = 0;
        let mut white = 1;
        for i in fen.chars() {
            if i.is_alphabetic() {
                res = 2_u64.pow((column + row * 8) as u32);
                //print_u64_bitboard(res);
            }
            if !i.is_uppercase() {
                white = 0;
            }
            else {
                white = 1
            }
            match i.to_ascii_lowercase() {
                'p' => {
                    column += 1;
                    _pawns[white] |= res;
                }
                'r' => {
                    column += 1;
                    _rooks[white] |= res;
                }
                'n' => {
                    column += 1;
                    _knights[white] |= res;
                }
                'q' => {
                    column += 1;
                    _queens[white] |= res;
                }
                'k' => {
                    column += 1;
                    _kings[white] |= res;
                }
                'b' => {
                    column += 1;
                    _bishops[white] |= res;
                }
                '/' => {
                    row-=1;
                    column = 0;
                }
                ' ' => {
                    break;
                }
                i if i.is_numeric() => {
                    column += i.to_digit(10).unwrap();
                }
                _ => {}
            }
        }
        let empty = !_pawns[0] & !_pawns[1] & !_knights[0] & !_knights[1] & !_bishops[0] & !_bishops[1] & !_rooks[0] & !_rooks[1] & !_queens[0] & !_queens[1] & !_kings[0] & !_kings[1];
        let black = _pawns[0]  | _knights[0] | _bishops[0] | _rooks[0]  | _queens[0] | _kings[0];
        let mut b = Board {
            pawns: _pawns,
            knights: _knights,
            bishops: _bishops,
            rooks: _rooks,
            queens: _queens,
            kings: _kings,
            black_pieces: black,
            white_pieces: (!(empty | black)),
            empty,
            white_long_c: true,
            white_short_c: true,
            black_long_c: true,
            black_short_c: true,
            watched_squares_black: 0,
            watched_squares_white: 0,
        };
        b.watched_squares_black = b.watched(false);
        b.watched_squares_white = b.watched(true);
        return b;
    }


    pub fn watched(&self, white: bool) -> u64 {
        return bishop::watched_by_b(&self, white) | king::watched_by_k(&self, white) | knight::watched_by_n(&self, white) | queen::watched_by_q(&self, white) | rook::watched_by_r(&self, white) | pawn::watched_by_p(&self, white);
    }

    pub fn h_and_vmoves(&self, s: usize, white: bool) -> u64 {
        let binary_s:u64 = 1<<s;
        let king = if white { self.kings[0] } else { self.kings[1] };
        let possibilities_horizontal: u64 = (((self.white_pieces | self.black_pieces) - king) - 2 * binary_s) ^ (((self.white_pieces | self.black_pieces) - king).reverse_bits() - 2 * binary_s.reverse_bits()).reverse_bits();
        let possibilities_vertical: u64 = ((((self.white_pieces | self.black_pieces) - king) & FILE_MASKS[s % 8]) - (2 * binary_s)) ^ ((((self.white_pieces | self.black_pieces) - king)& FILE_MASKS[s % 8]).reverse_bits() - (2 * binary_s.reverse_bits())).reverse_bits();
        return (possibilities_horizontal & RANK_MASKS[s / 8]) | (possibilities_vertical & FILE_MASKS[s % 8]);
    }

    pub fn d_and_anti_d_moves(&self, s: usize, white: bool) -> u64 {
        let binary_s:u64 = 1 << s;
        let king = if white { self.kings[0] } else { self.kings[1] };
        let possibilities_diagonal: u64 = ((((self.white_pieces | self.black_pieces) - king)&DIAGONAL_MASKS[(s / 8) + (s % 8)]) - (2 * binary_s)) ^ ((((self.white_pieces | self.black_pieces) - king)&DIAGONAL_MASKS[(s / 8) + (s % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
        let possibilities_anti_diagonal: u64 = ((((self.white_pieces | self.black_pieces) - king)&ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]) - (2 * binary_s)) ^ ((((self.white_pieces | self.black_pieces) - king)&ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
        return (possibilities_diagonal &DIAGONAL_MASKS[(s / 8) + (s % 8)]) | (possibilities_anti_diagonal &ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]);
    }
}
