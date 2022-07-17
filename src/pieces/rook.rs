use std::borrow::BorrowMut;

use crate::{Board, print_u64_bitboard};
use crate::consts::board_consts::*;
use crate::mv::Move;
use crate::pieces::common_moves;
use crate::pieces::common_moves::h_and_vmoves;
use crate::pieces::king;

pub fn possible_r(b: &Board, white: bool) -> Vec<Move> {
    let index = if white { 1 } else { 0 };
    let own = if white { b.white_pieces } else { b.black_pieces };
    let opp = if white { b.black_pieces - b.pieces[K_INDEX as usize] } else { b.white_pieces - b.pieces[(K_INDEX + 1) as usize] };
    let mut list: Vec<Move> = Vec::new();
    let rooks = b.pieces[(R_INDEX + index) as usize];

    if king::is_double_check(b.attackers) {
        return list;
    }
    for i in (rooks.trailing_zeros() as u8)..(64u8 - rooks.leading_zeros() as u8) {
        if (1 << i) & rooks != 0 {
            let moves = b.get_pinned_slide(i) & !own & common_moves::h_and_vmoves(i, opp, own);
            for i2 in 0u8..64u8 {
                if (1 << i2) & moves & b.push_mask != 0 {
                    list.push(
                        Move::new_move(i, i2, opp & (1 << i2) != 0)
                    );
                }
            }
        }
    }
    return list;
}

pub fn watched_by_r(b: &Board, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let own = if white { b.white_pieces } else { b.black_pieces };
    let opp = if white { b.black_pieces - b.pieces[K_INDEX as usize] } else { b.white_pieces - b.pieces[(K_INDEX + 1) as usize] };
    let mut moves = 0;

    let rooks = b.pieces[(R_INDEX + index) as usize];
    for i in 0u8..64u8 {
        if (1 << i) & rooks != 0 {
            moves |= common_moves::h_and_vmoves(i, opp, own);
        }
    }
    return moves;
}
