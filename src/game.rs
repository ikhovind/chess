pub use crate::game::board_consts::{RANK_4, RANK_8, FILE_MASKS, DIAGONAL_MASKS, ANTI_DIAGONAL_MASKS};
use crate::{mv, print_u64_bitboard};
use crate::game::board_consts::RANK_MASKS;
use crate::mv::{BISHOP, KNIGHT, Move, QUEEN, ROOK};

pub(crate) mod board_consts;

//[black, white]
pub struct Board {
    pub(crate) pawns: [u64; 2],
    pub(crate) knights: [u64; 2],
    pub(crate) bishops: [u64; 2],
    pub(crate) rooks: [u64; 2],
    pub(crate) queens: [u64; 2],
    pub(crate) kings: [u64; 2],
    black_pieces: u64,
    white_pieces: u64,
    empty: u64,
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
        return Board {pawns: _pawns, knights: _knights, bishops: _bishops, rooks: _rooks, queens: _queens, kings: _kings, black_pieces: black, white_pieces: (!(empty | black)), empty}
    }

    pub fn possible_p(&mut self, last_move: Move, white: usize) -> Vec<Move> {
        let mut list: Vec<Move> = Vec::new();
        let opposing_pieces = if white == 1 { self.black_pieces } else { self.white_pieces };
        let mut pawn_moves = (self.pawns[white] << 9) & (opposing_pieces) & (!RANK_8) & (!FILE_MASKS[0]); // capture right

        for i in 0..64 {
            if ((pawn_moves >> i) & 1) == 1 {
                list.push(Move::new_move(i - 9,i, true));
            }
        }

        pawn_moves = (self.pawns[white] << 7) & (opposing_pieces) & (!RANK_8) & (!FILE_MASKS[7]); // capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(i-7,i, true));
            }
        }
        pawn_moves=(self.pawns[white] << 8)&self.empty&!RANK_8;//move 1 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(i-8,i, false));
            }
        }
        pawn_moves=((self.pawns[white] << 16) & (self.empty & (self.empty << 8))) & RANK_4;//move 2 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_move(i - 16,i, false));
            }
        }
        //y1,y2,Promotion Type,"P"
        pawn_moves=(self.pawns[white] << 7)&opposing_pieces&RANK_8&!FILE_MASKS[0];//pawn promotion by capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_promotion(i - 7,i, true, QUEEN));
                list.push(Move::new_promotion(i - 7,i, true, ROOK));
                list.push(Move::new_promotion(i - 7,i, true, BISHOP));
                list.push(Move::new_promotion(i - 7,i, true, KNIGHT));
            }
        }

        pawn_moves=(self.pawns[white] << 9)&opposing_pieces&RANK_8&!FILE_MASKS[7];//pawn promotion by capture right
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_promotion(i - 9,i, true, QUEEN));
                list.push(Move::new_promotion(i - 9,i, true, ROOK));
                list.push(Move::new_promotion(i - 9,i, true, BISHOP));
                list.push(Move::new_promotion(i - 9,i, true, KNIGHT));
            }
        }

        pawn_moves=(self.pawns[white] << 8)&self.empty&RANK_8;//pawn promotion by move 1 forward
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_promotion(i - 8,i, false, QUEEN));
                list.push(Move::new_promotion(i - 8,i, false, ROOK));
                list.push(Move::new_promotion(i - 8,i, false, BISHOP));
                list.push(Move::new_promotion(i - 8,i, false, KNIGHT));
            }
        }
        // en passant
        pawn_moves = ((self.pawns[white] << 9) & (opposing_pieces << 8) & (!RANK_8) & (!FILE_MASKS[7])) & if Move::last_move_was_double_push(last_move) { 2_u64.pow(last_move.get_to_square() as u32) << 8} else { 0 };  // capture right
        for i in 0..64 {
            if ((pawn_moves >> i) & 1) == 1 {
                list.push(Move::new_ep(i - 9, i));
            }
        }

        pawn_moves = ((self.pawns[white] << 7) & (opposing_pieces << 8) & (!RANK_8) & (!FILE_MASKS[0])) & if Move::last_move_was_double_push(last_move) { 2_u64.pow(last_move.get_to_square() as u32) << 8} else { 0 }; // capture left
        for i in 0..64 {
            if ((pawn_moves>>i)&1)==1 {
                list.push(Move::new_ep(i - 7, i));
            }
        }
        return list;
    }

    pub fn possible_b(&self, white: bool) -> Vec<Move> {
        let mut opposing_pieces: u64 = self.white_pieces;
        let mut own_pieces = self.black_pieces;
        let mut index = 0;
        if white {
            opposing_pieces = self.black_pieces;
            own_pieces = self.white_pieces;
            index = 1;
        }
        let mut list: Vec<Move> = Vec::new();
        let bishops = self.bishops[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & bishops != 0 {
                let moves = self.d_and_anti_d_moves(i as usize) & !(own_pieces);
                for i2 in 0u8..64u8 {
                    if 2u64.pow(i2 as u32) & moves != 0 {
                        list.push(
                            Move::new_move(i, i2, opposing_pieces & 2_u64.pow(i2 as u32) != 0)
                        );
                    }
                }
            }
        }
        return list;
    }

    pub fn possible_r(&self, white: bool) -> Vec<Move> {
        let mut opposing_pieces: u64 = self.white_pieces;
        let mut own_pieces = self.black_pieces;
        let mut index = 0;
        if white {
            opposing_pieces = self.black_pieces;
            own_pieces = self.white_pieces;
            index = 1;
        }
        let mut list: Vec<Move> = Vec::new();
        let rooks = self.rooks[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & rooks != 0 {
                let moves = self.h_and_vmoves(i as usize) & !(own_pieces);
                for i2 in 0u8..64u8 {
                    if 2u64.pow(i2 as u32) & moves != 0 {
                        list.push(
                            Move::new_move(i, i2, opposing_pieces & 2_u64.pow(i2 as u32) != 0)
                        );
                    }
                }
            }
        }
        return list;
    }

    pub fn possible_q(&self, white: bool) -> Vec<Move> {
        let mut opposing_pieces: u64 = self.white_pieces;
        let mut own_pieces = self.black_pieces;
        let mut index = 0;
        if white {
            opposing_pieces = self.black_pieces;
            own_pieces = self.white_pieces;
            index = 1;
        }
        let mut list: Vec<Move> = Vec::new();
        let queens = self.queens[index];
        for i in 0u8..64u8 {
            if 2_u64.pow(i as u32) & queens != 0 {
                let moves = (self.d_and_anti_d_moves(i as usize) | self.h_and_vmoves(i as usize)) & !(own_pieces);
                for i2 in 0u8..64u8 {
                    if 2u64.pow(i2 as u32) & moves != 0 {
                        list.push(
                            Move::new_move(i, i2, opposing_pieces & 2_u64.pow(i2 as u32) != 0)
                        );
                    }
                }
            }
        }
        return list;
    }

    pub fn h_and_vmoves(&self, s: usize) -> u64 {
        let binary_s:u64 = 1<<s;
        let possibilities_horizontal: u64 = ((self.white_pieces | self.black_pieces) - 2 * binary_s) ^ ((self.white_pieces | self.black_pieces).reverse_bits() - 2 * binary_s.reverse_bits()).reverse_bits();
        let possibilities_vertical: u64 = (((self.white_pieces | self.black_pieces) & FILE_MASKS[s % 8]) - (2 * binary_s)) ^ (((self.white_pieces | self.black_pieces)& FILE_MASKS[s % 8]).reverse_bits() - (2 * binary_s.reverse_bits())).reverse_bits();
        return (possibilities_horizontal & RANK_MASKS[s / 8]) | (possibilities_vertical & FILE_MASKS[s % 8]);
    }

    pub fn d_and_anti_d_moves(&self, s: usize) -> u64 {
        let binary_s:u64 = 1 << s;
        let possibilities_diagonal: u64 = (((self.white_pieces | self.black_pieces)&DIAGONAL_MASKS[(s / 8) + (s % 8)]) - (2 * binary_s)) ^ (((self.white_pieces | self.black_pieces)&DIAGONAL_MASKS[(s / 8) + (s % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
        let possibilities_anti_diagonal: u64 = (((self.white_pieces | self.black_pieces)&ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]) - (2 * binary_s)) ^ (((self.white_pieces | self.black_pieces)&ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
        return (possibilities_diagonal &DIAGONAL_MASKS[(s / 8) + (s % 8)]) | (possibilities_anti_diagonal &ANTI_DIAGONAL_MASKS[(s / 8) + 7 - (s % 8)]);
    }
}
