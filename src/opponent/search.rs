use std::clone;
use crate::Board;
use crate::consts::board_consts::{B_INDEX, K_INDEX, N_INDEX, P_INDEX, Q_INDEX, R_INDEX};
use crate::move_gen::pieces;

const PAWN_VALUE: u32 = 100;
const QUEEN_VALUE: u32 = 900;
const KNIGHT_VALUE: u32 = 300;
const BISHOP_VALUE: u32 = 300;
const ROOK_VALUE: u32 = 500;


pub fn search_moves(b: &Board, depth: u8) -> i16 {
    if depth == 0 {
        return eval_static(b);
    }
    let mut best_evaluation = i16::MIN;
    if b.get_all_moves().len() != 0 {
        for mv in b.get_all_moves() {
            let mut evaluation = -search_moves(&b.clone().make_move(&mv), depth - 1);
            if evaluation > best_evaluation {
                best_evaluation = evaluation;
            }
        }
    }
    else {
        if pieces::king::get_attackers(&b, b.white_turn) != 0 {
            return -10000;
        }
        return 0;
    }
    return best_evaluation;
}

pub fn eval_static(b: &Board) -> i16 {
    let ix = if b.white_turn { 1 } else { 0 };
    return (b.pieces[P_INDEX + ix].count_ones() * PAWN_VALUE
        - b.pieces[P_INDEX + 1 - ix].count_ones() * PAWN_VALUE +
        b.pieces[Q_INDEX + ix].count_ones() * QUEEN_VALUE
        - b.pieces[Q_INDEX + 1 - ix].count_ones() * QUEEN_VALUE +
        b.pieces[N_INDEX + ix].count_ones() * KNIGHT_VALUE
        - b.pieces[N_INDEX + 1 - ix].count_ones() * KNIGHT_VALUE +
        b.pieces[B_INDEX + ix].count_ones() * BISHOP_VALUE
        - b.pieces[B_INDEX + 1 - ix].count_ones() * BISHOP_VALUE +
        b.pieces[R_INDEX + ix].count_ones() * ROOK_VALUE
        - b.pieces[R_INDEX + 1 - ix].count_ones() * ROOK_VALUE) as i16;
}