use std::cmp::{max, min};

use crate::consts::board_consts::{ANTI_DIAGONAL_MASKS, DIAGONAL_MASKS, FILE_MASKS};
use crate::consts::board_consts::*;
use crate::mv::Move;
use crate::move_gen::pieces::*;
use crate::move_gen::pieces::bishop;
use crate::move_gen::pieces::common_moves::{d_and_anti_d_moves, h_and_vmoves};
use crate::print_u64_bitboard;

//[black, white]
//[black short, black long, white short, white long]
#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub pieces: [u64; 12],
    pub castle_rights: [bool; 4],
    pub white_turn: bool,
    pub last_move: Move,
    pub push_mask: u64,
    pub pinned_pieces: u64,
}

impl Board {
    pub fn from_fen(fen: String) -> Board {
        let mut _pawns = [0, 0];
        let mut _bishops = [0, 0];
        let mut _rooks = [0, 0];
        let mut _knights = [0, 0];
        let mut _queens = [0, 0];
        let mut _kings = [0, 0];

        let mut column: u32 = 0;
        let mut row = 7;
        let mut res = 0;
        let mut white;
        for i in fen.chars() {
            if i.is_alphabetic() {
                res = 2_u64.pow((column + row * 8) as u32);
                //print_u64_bitboard(res);
            }
            if !i.is_uppercase() {
                white = 0;
            } else {
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
                    row -= 1;
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
        let mut b = Board {
            pieces: [_pawns[0], _pawns[1], _knights[0], _knights[1], _bishops[0], _bishops[1], _rooks[0], _rooks[1], _queens[0], _queens[1], _kings[0], _kings[1]],
            castle_rights: [true, true, true, true],
            white_turn: false,
            last_move: Move::new_move(0, 0, false),
            push_mask: u64::MAX,
            pinned_pieces: 0,
        };
        b.update_metadata(&Move::new_move(0, 0, false));
        return b;
    }


    pub fn get_white_pieces(&self) -> u64 {
        return self.pieces[P_INDEX + 1] | self.pieces[N_INDEX + 1] | self.pieces[B_INDEX + 1] | self.pieces[R_INDEX + 1] | self.pieces[Q_INDEX + 1] | self.pieces[K_INDEX + 1];
    }

    pub fn get_black_pieces(&self) -> u64 {
        return self.pieces[P_INDEX] | self.pieces[N_INDEX] | self.pieces[B_INDEX] | self.pieces[R_INDEX] | self.pieces[Q_INDEX] | self.pieces[K_INDEX];
    }

    pub fn get_empty(&self) -> u64 {
        return !(self.get_white_pieces() | self.get_black_pieces());
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

    pub fn make_move(&mut self, mv: &Move) -> &mut Board {
        let mv_type = mv.from & TYPE_MASK | ((mv.to & TYPE_MASK) >> 2);
        let color: u8 = if self.white_turn { 1 } else { 0 };
        let from_sq = 1u64 << (mv.from & MOVE_MASK);
        let to_sq = 1u64 << (mv.to & MOVE_MASK);
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
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[P_INDEX + color as usize] += to_sq;
            }
            TAKES => {
                for i in (color as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & from_sq != 0 {
                        for i2 in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                            if self.pieces[i2] & to_sq != 0 {
                                self.pieces[i2] -= to_sq;
                                self.pieces[i] += to_sq;
                                self.pieces[i] -= from_sq;
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
                    } else {
                        to_sq >> 8
                    };
                self.pieces[P_INDEX + (1 - color) as usize] -= opp;
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[P_INDEX + color as usize] += to_sq;
            }
            PROM_Q => {
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[Q_INDEX + color as usize] += to_sq;
            }
            PROM_R => {
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[R_INDEX + color as usize] += to_sq;
            }
            PROM_B => {
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[B_INDEX + color as usize] += to_sq;
            }
            PROM_N => {
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[N_INDEX + color as usize] += to_sq;
            }
            TAKE_PROM_Q => {
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[Q_INDEX + color as usize] += to_sq;

                for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] -= to_sq;
                        break;
                    }
                }
            }
            TAKE_PROM_R => {
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[R_INDEX + color as usize] += to_sq;
                for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] -= to_sq;
                        break;
                    }
                }
            }
            TAKE_PROM_B => {
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[B_INDEX + color as usize] += to_sq;

                for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] -= to_sq;
                        break;
                    }
                }
            }
            TAKE_PROM_N => {
                self.pieces[P_INDEX + color as usize] -= from_sq;
                self.pieces[N_INDEX + color as usize] += to_sq;

                for i in ((1 - color) as usize..self.pieces.len()).step_by(2) {
                    if self.pieces[i] & to_sq != 0 {
                        self.pieces[i] -= to_sq;
                        break;
                    }
                }
            }
            SHORT_CASTLE => {
                self.pieces[K_INDEX + color as usize] -= from_sq;
                self.pieces[K_INDEX + color as usize] += to_sq;

                self.pieces[R_INDEX + color as usize] -= to_sq << 1;
                self.pieces[R_INDEX + color as usize] += from_sq << 1;


                self.castle_rights[(color * 2 + 1) as usize] = false;
                self.castle_rights[(color * 2) as usize] = false;
            }
            LONG_CASTLE => {
                self.pieces[K_INDEX + color as usize] -= from_sq;
                self.pieces[K_INDEX + color as usize] += to_sq;

                self.pieces[R_INDEX + color as usize] += to_sq << 1;
                self.pieces[R_INDEX + color as usize] -= to_sq >> 2;
                self.castle_rights[(color * 2 + 1) as usize] = false;
                self.castle_rights[(color * 2) as usize] = false;
            }
            _ => {
                log::error!("illegal move??: {}", mv_type);
            }
        }
        self.update_metadata(mv);
        return self;
    }

    fn update_castling_rights(&mut self, white: bool) {
        let color = if white { 1 } else { 0 };
        // short castle
        let short_rook = if white { WHITE_SHORT_ORG_ROOK } else { BLACK_SHORT_ORG_ROOK };
        let long_rook = if white { WHITE_LONG_ORG_ROOK } else { BLACK_LONG_ORG_ROOK };
        let king = if white { WHITE_KING } else { BLACK_KING };
        if self.pieces[(R_INDEX + color) as usize] & short_rook == 0 || self.pieces[(K_INDEX + color) as usize] & king == 0 {
            self.castle_rights[(color * 2) as usize] = false;
        }
        if self.pieces[(R_INDEX + color) as usize] & long_rook == 0 || self.pieces[(K_INDEX + color) as usize] & king == 0 {
            self.castle_rights[(color * 2 + 1) as usize] = false;
        }
    }

    fn update_metadata(&mut self, mv: &Move) {
        self.white_turn = !self.white_turn;
        self.update_castling_rights(self.white_turn);
        self.update_castling_rights(!self.white_turn);
        self.last_move = *mv;

        self.pinned_pieces = self.get_pinned_pieces(self.white_turn);

        // set push mask
        let index = if self.white_turn { 1 } else { 0 };
        let attackers = king::get_attackers(self, self.white_turn);
        if attackers != 0 {
            if king::is_double_check(attackers) {
                self.push_mask = 0;
            }
            else {
                // hvis brikken som ble flytta er en glider
                if (1u64 << attackers.trailing_zeros()) & (self.pieces[(R_INDEX + 1 - index) as usize] | self.pieces[(Q_INDEX + 1 - index) as usize] | self.pieces[(B_INDEX + 1 - index) as usize]) != 0 {
                    self.push_mask = self.ray_between(attackers.trailing_zeros() as u8, self.pieces[(K_INDEX + index) as usize].trailing_zeros() as u8);
                } else {
                    self.push_mask = 1 << attackers.trailing_zeros();
                }
            }
        } else {
            self.push_mask = u64::MAX;
        }
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        let mut rook = rook::possible_r(self, self.white_turn);
        rook.append(&mut knight::possible_n(self, self.white_turn));
        rook.append(&mut bishop::possible_b(self, self.white_turn));
        rook.append(&mut queen::possible_q(self, self.white_turn));
        rook.append(&mut king::possible_k(self, self.white_turn));
        rook.append(&mut pawn::possible_p(self, self.white_turn));
        return rook.clone();
    }

    pub fn get_num_moves(self, depth: u32) -> u64 {
        return self.get_num_moves_inner(depth, depth);
    }

    fn get_num_moves_inner(&self, depth: u32, initial: u32) -> u64 {
        let mut sum = 0;
        if depth == 1 {
            return self.get_all_moves().len() as u64;
        }
        for nw in self.get_all_moves() {
            let res = self.clone().make_move(&nw).get_num_moves_inner(depth - 1, initial);
            sum += res;
            if depth == initial {
                println!("{}: {}", nw.to_string(), res);
            }
        }
        return sum;
    }

    pub fn ray_between(&self, attacker: u8, piece_square: u8) -> u64 {
        // same column
        let mut max = max(attacker, piece_square);
        let min = min(attacker, piece_square);
        let mut ray = 0;
        if max % 8 == min % 8 {
            max -= 8;
            while max != min {
                ray |= 1 << max;
                max -= 8;
            }
        }
        // same row
        else if max / 8 == min / 8 {
            max -= 1;
            while max != min {
                ray |= 1 << max;
                max -= 1;
            }
        }
        // diagonal
        else {
            // to the left
            if max % 8 < min % 8 {
                max -= 7;
                while max != min {
                    ray |= 1 << max;
                    max -= 7;
                }
            }
            // to the right
            else {
                max -= 9;
                while max != min {
                    ray |= 1 << max;
                    max -= 9;
                }
            }
        }
        return ray | (1 << attacker);
    }

    pub fn get_pinned_pieces(&self, white: bool) -> u64 {
        let index = if white { 1 } else { 0 };
        let attacking_color = if white { self.get_black_pieces() } else { self.get_white_pieces() };
        let def_color = if white { self.get_white_pieces() } else { self.get_black_pieces() };
        let king_square  = self.pieces[K_INDEX + index].trailing_zeros() as u8;
        let opp_diags = self.pieces[B_INDEX + 1 - index] | self.pieces[Q_INDEX + 1 - index];
        let opp_line = self.pieces[R_INDEX + 1 - index] | self.pieces[Q_INDEX + 1 - index];
        let king_diag = DIAGONAL_MASKS[((king_square as usize) / 8) + ((king_square as usize) % 8)];
        let king_anti_diag = ANTI_DIAGONAL_MASKS[((7 - king_square % 8) + king_square / 8) as usize];

        let mut pinned_pieces = 0;
        for i in (opp_diags | opp_line).trailing_zeros() as u8..(64u8 - (opp_diags | opp_line).leading_zeros() as u8) {
            if (1 << i) & opp_line != 0 {
                if i % 8 == king_square % 8 {
                    pinned_pieces |= h_and_vmoves(i, def_color, attacking_color) & h_and_vmoves(king_square, attacking_color, def_color);
                } else if (i / 8) == (king_square / 8) {
                    pinned_pieces |=  h_and_vmoves(i, def_color, attacking_color) & h_and_vmoves(king_square, attacking_color, def_color);
                }
            }
            if (1 << i) & opp_diags != 0 {
                if (1 << i) & king_diag != 0 {
                    pinned_pieces |= d_and_anti_d_moves(i, def_color, attacking_color) & d_and_anti_d_moves(king_square, attacking_color, def_color);
                } else if (1 << i) & king_anti_diag != 0 {
                    pinned_pieces |= d_and_anti_d_moves(i, def_color, attacking_color) & d_and_anti_d_moves(king_square, attacking_color, def_color);
                }
            }
        }
        return pinned_pieces & def_color;
    }

    pub fn get_pinning_ray(self, king_square: u8, piece_square: u8) -> u64 {
        // same column
        if king_square % 8 == piece_square % 8 {
            return FILE_MASKS[(king_square % 8) as usize];
        }
        // same row
        if king_square / 8 == piece_square / 8 {
            return RANK_MASKS[(king_square / 8) as usize];
        }
        // diagonal
        // to the left
        if u8::abs_diff(piece_square, king_square) % 9 == 0 {
            return ANTI_DIAGONAL_MASKS[((7 - king_square % 8) + king_square / 8) as usize];
        }
        // to the right
        return if u8::abs_diff(piece_square, king_square) % 7 == 0 {
            DIAGONAL_MASKS[(king_square % 8 + king_square / 8) as usize]
        } else {
            0
        }
    }

    pub fn get_pinned_slide(&self, i: u8) -> u64 {
        let index = if self.white_turn { 1 } else { 0 };
        return if self.pinned_pieces & (1 << i) != 0 {
            self.get_pinning_ray(self.pieces[K_INDEX + index].trailing_zeros() as u8, i)
        } else {
            u64::MAX
        };
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.pieces == other.pieces
            && self.white_turn == other.white_turn
    }
}
