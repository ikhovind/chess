use std::clone;
use std::cmp::max;
use std::fmt::format;
use log::log;
use num_format::Locale::be;
use crate::{Board, eval, Move, print_u64_bitboard};
use crate::consts::board_consts::{B_INDEX, K_INDEX, N_INDEX, N_INF, P_INDEX, Q_INDEX, R_INDEX};
use crate::move_gen::pieces;

const PAWN_VALUE: u32 = 100;
const QUEEN_VALUE: u32 = 900;
const KNIGHT_VALUE: u32 = 300;
const BISHOP_VALUE: u32 = 300;
const ROOK_VALUE: u32 = 500;


pub fn search_moves(b: &Board, depth: u8, mut best_current_side: i16, best_opponent: i16) -> i16 {
    let moves = b.get_all_moves();
    if moves.len() == 0 {
        if pieces::king::get_attackers(b, b.white_turn) != 0 {
            return N_INF;
        }
        return 0;
    }
    else {
        if depth == 0 {
            return quiescence_search(&b, best_current_side, best_opponent);
        }
        for mv in moves {
            let evaluation = -search_moves(b.clone().make_move(&mv), depth - 1, -best_opponent, -best_current_side);
            // opponent has a better choice, can prune
            if evaluation >= best_opponent {
                return best_opponent;
            }
            best_current_side = max(best_current_side, evaluation);
        }
        return best_current_side;
    }
}

pub fn count_material(b: &Board) -> i16 {
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

fn quiescence_search(b: &Board, mut best_current_side: i16, best_opponent: i16) -> i16 {
    let mut eval = count_material(&b);
    if eval >= best_opponent {
        return best_opponent;
    }
    best_current_side = max(best_current_side, eval);
    //log::info!("quiesence eval: {}", mv);

    for mv in b.get_all_moves() {
        if mv.is_capture() {
            eval = -quiescence_search(b.clone().make_move(&mv), -best_opponent, -best_current_side);
            // opponent has a better choice, can prune
            if eval >= best_opponent {
                return best_opponent;
            }
            best_current_side = max(best_current_side, eval);
        }
    }
    return best_current_side;
}