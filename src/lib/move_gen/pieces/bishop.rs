use crate::{Board};
use crate::consts::board_consts::*;
use crate::mv::Move;
use crate::move_gen::pieces::common_moves;
use crate::move_gen::pieces::common_moves::add_moves_to_list;

pub fn watched_by_b(b: &Board, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let own = if white { b.get_white_pieces() } else { b.get_black_pieces() };
    let opp = if white { b.get_black_pieces() - b.pieces[K_INDEX as usize] } else { b.get_white_pieces() - b.pieces[(K_INDEX + 1) as usize] };
    let mut moves = 0;

    let bishops = b.pieces[(B_INDEX + index) as usize];
    for i in (bishops.trailing_zeros() as u8)..(64u8 - bishops.leading_zeros() as u8) {
        if (1 << i) & bishops != 0 {
            moves |= common_moves::d_and_anti_d_moves(i, opp, own);
        }
    }
    moves
}

pub fn possible_b(b: &Board, captures: bool) -> Vec<Move> {
    let index = if b.white_turn { 1 } else { 0 };
    let own = if b.white_turn { b.get_white_pieces() } else { b.get_black_pieces() };
    let opp = if b.white_turn { b.get_black_pieces() - b.pieces[K_INDEX as usize] } else { b.get_white_pieces() - b.pieces[(K_INDEX + 1) as usize] };
    let cap_mask = if captures { opp } else { u64::MAX };

    let bishops = b.pieces[(B_INDEX + index) as usize];
    let mut list: Vec<Move> = Vec::with_capacity((bishops.count_ones() * 8) as usize);
    for i in (bishops.trailing_zeros())..(64 - bishops.leading_zeros()) {
        if (1 << i) & bishops != 0 {
            let moves = cap_mask & b.get_pinned_slide(i as u8) & b.push_mask & !own & common_moves::d_and_anti_d_moves(i as u8, opp, own);
            add_moves_to_list(opp, &mut list, i, moves);
        }
    }
    list
}

