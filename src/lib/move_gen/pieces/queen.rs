use crate::{Board};
use crate::move_gen::pieces::common_moves;
use crate::consts::board_consts::K_INDEX;
use crate::consts::board_consts::*;
use crate::move_gen::pieces::common_moves::add_moves_to_list;
use crate::mv::Move;

pub fn possible_q(b: &Board, captures: bool) -> Vec<Move> {
    let index = if b.white_turn { 1 } else { 0 };
    let own = if b.white_turn { b.get_white_pieces() } else { b.get_black_pieces()};
    let opp = if b.white_turn { b.get_black_pieces() - b.pieces[K_INDEX as usize] } else { b.get_white_pieces() - b.pieces[(K_INDEX + 1) as usize]};
    let capture_mask = if captures { opp } else { u64::MAX };
    let mut list: Vec<Move> = Vec::new();
    let queens = b.pieces[(Q_INDEX + index) as usize];
    list.reserve((queens.count_ones() * 20) as usize);
    for i in (queens.trailing_zeros())..(64 - queens.leading_zeros()) {
        if (1 << i) & queens != 0 {
            let moves = capture_mask & b.push_mask & b.get_pinned_slide(i as u8) & !own
                & (common_moves::d_and_anti_d_moves(i as u8, opp, own)
                | common_moves::h_and_vmoves(i as u8, opp, own));
            add_moves_to_list(opp, &mut list, i, moves);
        }
    }
    list
}

pub fn watched_by_q(b: &Board, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let own = if white { b.get_white_pieces() } else { b.get_black_pieces() };
    let opp = if white { b.get_black_pieces() - b.pieces[K_INDEX as usize] } else { b.get_white_pieces() - b.pieces[(K_INDEX + 1) as usize] };
    let mut moves = 0;

    let queens = b.pieces[(Q_INDEX + index) as usize];
    for i in (queens.trailing_zeros() as u8)..(64u8 - queens.leading_zeros() as u8) {
        if (1 << i) & queens != 0 {
            moves |= common_moves::d_and_anti_d_moves(i, opp, own)
                | common_moves::h_and_vmoves(i, opp, own);
        }
    }
    moves
}