use std::collections::BinaryHeap;
use crate::{Board};
use crate::consts::board_consts::*;
use crate::mv::Move;
use crate::move_gen::pieces::common_moves;
use crate::move_gen::pieces::common_moves::add_moves_to_list;

pub fn possible_r(b: &Board, captures: bool) -> Vec<Move> {
    let index = if b.white_turn { 1 } else { 0 };
    let own = if b.white_turn { b.get_white_pieces() } else { b.get_black_pieces() };
    let opp = if b.white_turn { b.get_black_pieces() - b.pieces[K_INDEX] } else { b.get_white_pieces() - b.pieces[K_INDEX + 1] };
    let cap_mask = if captures { opp } else { u64::MAX };
    let rooks = b.pieces[(R_INDEX + index) as usize];
    let mut list: Vec<Move> = Vec::new();

    list.reserve((rooks.count_ones() * 8) as usize);
    for i in (rooks.trailing_zeros())..(64 - rooks.leading_zeros()) {
        if (1 << i) & rooks != 0 {
            let moves = cap_mask & b.push_mask & b.get_pinned_slide(i as u8) & !own & common_moves::h_and_vmoves(i as u8, opp, own);
            add_moves_to_list(opp, &mut list, i, moves);
        }
    }
    list
}


pub fn watched_by_r(b: &Board, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let own = if white { b.get_white_pieces() } else { b.get_black_pieces() };
    let opp = if white { b.get_black_pieces() - b.pieces[K_INDEX as usize] } else { b.get_white_pieces() - b.pieces[(K_INDEX + 1) as usize] };
    let mut moves = 0;

    let rooks = b.pieces[(R_INDEX + index) as usize];
    for i in (rooks.trailing_zeros() as u8)..(64u8 - rooks.leading_zeros() as u8) {
        if (1 << i) & rooks != 0 {
            moves |= common_moves::h_and_vmoves(i, opp, own);
        }
    }
    moves
}
