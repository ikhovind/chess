use std::cmp::{max, min};
pub use crate::consts::board_consts::{FILE_MASKS, DIAGONAL_MASKS, ANTI_DIAGONAL_MASKS};
pub use crate::consts::board_consts::*;
use crate::{pieces, print_u64_bitboard};
use crate::mv::{BISHOP, KNIGHT, Move, QUEEN, ROOK};
use crate::pieces::*;
use crate::pieces::bishop;
use crate::pieces::common_moves::{d_and_anti_d_moves, h_and_vmoves};
use crate::pieces::king::is_double_check;

//[black, white]
//[black short, black long, white short, white long]
#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub(crate) pieces: [u64; 12],
    pub(crate) black_pieces: u64,
    pub(crate) white_pieces: u64,
    pub(crate) empty: u64,
    pub(crate) castle_rights: [bool; 4],
    pub(crate) white_long_c: bool,
    pub(crate) white_short_c: bool,
    pub(crate) black_long_c: bool,
    pub(crate) black_short_c: bool,
    pub(crate) watched_squares_white: u64,
    pub(crate) watched_squares_black: u64,
    pub(crate) white_turn: bool,
    pub(crate) last_move: Move,
    pub(crate) attackers: u64,
    pub(crate) push_mask: u64,
    pub(crate) pinned_pieces: u64,
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
            pieces: [_pawns[0],_pawns[1],_knights[0],_knights[1],_bishops[0],_bishops[1],_rooks[0],_rooks[1],_queens[0] ,_queens[1] ,_kings[0] , _kings[1]],
            black_pieces: black,
            white_pieces: (!(empty | black)),
            empty,
            castle_rights: [true, true, true, true],
            white_long_c: true,
            white_short_c: true,
            black_long_c: true,
            black_short_c: true,
            watched_squares_black: 0,
            watched_squares_white: 0,
            white_turn: false,
            last_move: Move::new_move(0,0, false),
            attackers: 0,
            push_mask: u64::MAX,
            pinned_pieces: 0,
        };
        b.watched_squares_black = b.watched(false);
        b.watched_squares_white = b.watched(true);
        b.attackers = king::get_attackers(&b, b.white_turn);
        b.update_metadata(Move::new_move(0, 0, false));
        return b;
    }

    pub fn watched(&self, white: bool) -> u64 {
        return
            bishop::watched_by_b(&self, white)
            | king::watched_by_k(&self, white)
            | knight::watched_by_n(&self, white)
            | queen::watched_by_q(&self, white)
            | rook::watched_by_r(&self, white)
            | pawn::watched_by_p(&self, white);
    }

    pub fn make_move(&mut self, mv: Move) -> &mut Board {
        let mv_type = ((mv.from >> 4) & 0b1100) | (mv.to >> 6);
        let color: u8 = if self.white_turn { 1 } else { 0 };
        let from_sq = 1 << (mv.from & MOVE_MASK);
        let to_sq = 1 << (mv.to & MOVE_MASK);
        match mv_type {
            NORMAL_MOVE => {
                for i in (color as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & from_sq != 0 {
                        self.pieces[i] += to_sq;
                        self.pieces[i] -= from_sq;
                        break;
                    }
                }
            }
            DOUBLE_PAWN => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(P_INDEX + color) as usize] += to_sq;
            }
            TAKES => {
                // white short castle
                for i in (color as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] += to_sq;
                        self.pieces[i] -= from_sq;
                        for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                            if self.pieces[i] & to_sq != 0 {
                                self.pieces[i] -= to_sq;
                                break;
                            }
                        }
                    }
                }
            }
            EN_PASSANT => {
                let opp =
                    if to_sq & RANK_MASKS[2] != 0 {
                        to_sq << 8
                    }
                    else {
                        to_sq >> 8
                    };
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(P_INDEX + color) as usize] += to_sq;
                self.pieces[(P_INDEX + (1 - color)) as usize] -= opp;
            }
            PROM_Q => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(Q_INDEX + color) as usize] += to_sq;
            }
            PROM_R => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(R_INDEX + color) as usize] += to_sq;
            }
            PROM_B => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(B_INDEX + color) as usize] += to_sq;
            }
            PROM_N => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(N_INDEX + color) as usize] += to_sq;
            }
            TAKE_PROM_Q => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(Q_INDEX + color) as usize] += to_sq;

                for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] -= to_sq;
                        break;
                    }
                }
            }
            TAKE_PROM_R => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(R_INDEX + color) as usize] += to_sq;
                for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] -= to_sq;
                        break;
                    }
                }
            }
            TAKE_PROM_B => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(B_INDEX + color) as usize] += to_sq;

                for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] -= to_sq;
                        break;
                    }
                }
            }
            TAKE_PROM_N => {
                self.pieces[(P_INDEX + color) as usize] -= from_sq;
                self.pieces[(N_INDEX + color) as usize] -= to_sq;

                for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] -= to_sq;
                        break;
                    }
                }
            }
            SHORT_CASTLE => {
                self.pieces[(K_INDEX + color) as usize] -= from_sq;
                self.pieces[(K_INDEX + color) as usize] += to_sq;
                self.pieces[(R_INDEX + color) as usize] -= from_sq << 1;
                self.castle_rights[(color * 2 + 1) as usize] = false;
                self.castle_rights[(color * 2) as usize] = false;
            }
            LONG_CASTLE => {
                self.pieces[(K_INDEX + color) as usize] -= from_sq;
                self.pieces[(K_INDEX + color) as usize] += to_sq;
                self.pieces[(R_INDEX + color) as usize] -= from_sq >> 2;
                self.castle_rights[(color * 2 + 1) as usize] = false;
                self.castle_rights[(color * 2) as usize] = false;
            }
            _ => {
                eprintln!("illegal move??: {}", mv_type);
            }
        }
        self.update_metadata(mv);
        return self;
    }

    fn update_castling_rights(&mut self, white: bool) {
        let color = if white { 1 } else { 0 };
        // short castle
        let short_rook = if white { WHITE_SHORT_CASTLE_ROOK } else { BLACK_SHORT_CASTLE_KING };
        let long_rook = if white { WHITE_LONG_CASTLE_ROOK } else { BLACK_LONG_CASTLE_ROOK };
        let king = if white { WHITE_KING } else { BLACK_KING };
        if self.pieces[(R_INDEX + color) as usize] & short_rook == 0 || self.pieces[(K_INDEX + color) as usize] & king == 0 {
            self.castle_rights[(color * 2) as usize] = false;
        }
        if self.pieces[(R_INDEX + color) as usize] & long_rook == 0 || self.pieces[(K_INDEX + color) as usize] & king == 0 {
            self.castle_rights[(color * 2 + 1) as usize] = false;
        }
    }

    fn update_metadata(&mut self, mv: Move) {
        self.black_pieces = 0;
        self.white_pieces = 0;

        for i in (0usize..self.pieces.len()).step_by(2) {
            self.black_pieces |= self.pieces[i];
        }

        for i in (1usize..self.pieces.len()).step_by(2) {
            self.white_pieces |= self.pieces[i];
        }
        self.empty = !self.white_pieces & !self.black_pieces;
        self.watched_squares_black = self.watched(false);
        self.watched_squares_white = self.watched(true);

        self.update_castling_rights(self.white_turn);
        self.white_turn = !self.white_turn;
        self.last_move = mv;

        self.pinned_pieces = self.get_pinned_pieces(self.white_turn);

        // set push mask
        let index = if self.white_turn { 1 } else { 0 };
        self.attackers = king::get_attackers(self, self.white_turn);
        if !king::is_double_check(self.attackers) && self.attackers != 0 {
            self.push_mask = 0;
            if (1 << (mv.to & MOVE_MASK)) & (self.pieces[(R_INDEX + 1) as usize] | self.pieces[(Q_INDEX + 1) as usize] | self.pieces[(B_INDEX + 1) as usize]) != 0 {
                self.push_mask = self.ray_between( (63 - self.attackers.leading_zeros()) as u8, (63 - self.pieces[(K_INDEX + index) as usize].leading_zeros()) as u8);
            }
            else {
                self.push_mask = 1 << (mv.to & MOVE_MASK);
            }
        }
        else {
            self.push_mask = u64::MAX;
        }
    }

    pub fn get_all_moves(&self) -> Vec<Move> {

        /*
        println!("rook: {}", pieces::rook::possible_r(self, self.white_turn).len());
        println!("knight: {}", pieces::knight::possible_n(self, self.white_turn).len());
        println!("bishop: {}", pieces::bishop::possible_b(self, self.white_turn).len());
        println!("queen: {}", pieces::queen::possible_q(self, self.white_turn).len());
        println!("king: {}", pieces::king::possible_k(self, self.white_turn).len());
        println!("pawn: {}", pieces::pawn::possible_p(self, self.white_turn).len());
         */

        let mut rook = rook::possible_r(self, self.white_turn);
        rook.append(&mut knight::possible_n(self, self.white_turn));
        rook.append(&mut bishop::possible_b(self, self.white_turn));
        rook.append(&mut queen::possible_q(self, self.white_turn));
        rook.append(&mut king::possible_k(self, self.white_turn));
        rook.append(&mut pawn::possible_p(self, self.white_turn));
        return rook;
    }

    pub fn get_num_moves(self, depth: u32) -> u64 {
        let mut sum = 0;
        let b2 = self.clone();
        if depth == 1 {
            return self.get_all_moves().len() as u64;
        }
        for nw in self.get_all_moves() {
            let &mut test = self.clone().make_move(nw);
            let &mut test2 = self.clone().make_move(nw);
            sum += test.get_num_moves(depth - 1);
            if depth == 4 { println!("{}: {}", nw.to_string(), test2.get_num_moves(depth - 1));}
        }
        return sum;
    }

    pub fn ray_between(self, attacker: u8, piece_square: u8) -> u64 {
        // same column
        let mut max = max(attacker, piece_square);
        let min = min(attacker, piece_square);
        let mut ray = 0;
        if max % 8 == min % 8 {
            max -= 8;
            while max != min {
                ray |= (1 << max);
                max -= 8;
            }
        }
        // same row
        else if max / 8 == min / 8 {
            max -= 1;
            while max != min {
                ray |= (1 << max);
                max -= 1;
            }
        }
        // diagonal
        else {
            // to the left
            if max % 8 < min % 8 {
                max -= 7;
                while max != min {
                    ray |= (1 << max);
                    max -= 7;
                }
            }
            // to the right
            else {
                max -= 9;
                while max != min {
                    ray |= (1 << max);
                    max -= 9;
                }
            }
        }
        return ray | (1 << attacker);
    }

    pub fn get_pinned_pieces(self, white: bool) -> u64 {
        // todo kanskje bug siden opp fjerner kongen her men blir brukt som own
        let index = if white { 1 } else { 0 };
        let attacking_color = if white { self.black_pieces } else { self.white_pieces };
        let def_color = if white { self.white_pieces } else { self.black_pieces };
        let king_square: u8 = (63 - self.pieces[(K_INDEX + index) as usize].leading_zeros()) as u8;
        let opp_diags = self.pieces[(B_INDEX + 1 - index) as usize] | self.pieces[(Q_INDEX + 1 - index) as usize];
        let opp_line = self.pieces[(R_INDEX + 1 - index) as usize] | self.pieces[(Q_INDEX + 1 - index) as usize];
        let king_diag =  DIAGONAL_MASKS[(king_square % 8 + king_square / 8) as usize];
        let king_anti_diag =  ANTI_DIAGONAL_MASKS[(king_square % 8 + king_square / 8) as usize];
        let mut pinned_pieces = 0;
        for i in 0u8..64u8 {
            if (1 << i) & opp_line != 0 {
                if i % 8 == king_square % 8  {
                    pinned_pieces |= (FILE_MASKS[(king_square % 8) as usize] & h_and_vmoves(i, def_color, attacking_color) & h_and_vmoves(king_square, attacking_color, def_color));
                }
                else if (i / 8) == (king_square / 8)  {
                    //print_u64_bitboard(h_and_vmoves(i, opp, own));
                    pinned_pieces |= (RANK_MASKS[(king_square / 8) as usize] & h_and_vmoves(i, def_color, attacking_color) & h_and_vmoves(king_square, attacking_color, def_color));
                }
            }
            if (1 << i) & opp_diags != 0 {
                if (1 << i) & king_diag != 0 {
                    pinned_pieces |= king_diag & d_and_anti_d_moves(i, def_color, attacking_color) & d_and_anti_d_moves(king_square, attacking_color, def_color);
                }
                else if (1 << i) & king_anti_diag != 0 {
                    pinned_pieces |= king_anti_diag & d_and_anti_d_moves(i, def_color, attacking_color) & d_and_anti_d_moves(king_square, attacking_color, def_color);
                }
            }
        }
        return pinned_pieces;
    }
    pub fn get_pinning_ray(self, king_square: u8, piece_square: u8) -> u64 {
        // same column
        let mut max = max(king_square, piece_square);
        let min = min(king_square, piece_square);
        let mut ray = 0;
        if max % 8 == min % 8 {
            return FILE_MASKS[(max % 8) as usize];
        }
        // same row
        else if max / 8 == min / 8 {
            return RANK_MASKS[(max / 8) as usize];
        }
        // diagonal
        // to the left
        else if (max - min) % 9 == 0 {
            return ANTI_DIAGONAL_MASKS[(max % 8 + max / 8) as usize];
        }
        // to the right
        else if (max - min) % 7 == 0 {
            return DIAGONAL_MASKS[(max % 8 + max / 8) as usize];
        }
        else {
            return 0;
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.pieces == other.pieces
        || self.white_turn == other.white_turn
    }
}
