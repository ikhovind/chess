use crate::{Board};
use crate::consts::board_consts::*;
use crate::mv::Move;
use crate::move_gen::pieces::common_moves;

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
    return moves;
}

pub fn possible_b(b: &Board, white: bool) -> Vec<Move> {
    let index = if white { 1 } else { 0 };
    let own = if white { b.get_white_pieces() } else { b.get_black_pieces() };
    let opp = if white { b.get_black_pieces() - b.pieces[K_INDEX as usize] } else { b.get_white_pieces() - b.pieces[(K_INDEX + 1) as usize] };

    let mut list: Vec<Move> = Vec::new();
    let bishops = b.pieces[(B_INDEX + index) as usize];
    for i in (bishops.trailing_zeros())..(64 - bishops.leading_zeros()) {
        if (1 << i) & bishops != 0 {
            let moves = b.get_pinned_slide(i as u8) & b.push_mask & !own & common_moves::d_and_anti_d_moves(i as u8, opp, own);
            for i2 in (moves.trailing_zeros())..(64 - moves.leading_zeros()) {
                if (1 << i2) & moves != 0 {
                    list.push(
                        Move::new_move(i as u8, i2 as u8, opp & (1 << i2) != 0)
                    );
                }
            }
        }
    }
    return list;
}

