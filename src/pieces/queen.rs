use crate::{Board, Move, print_u64_bitboard};
use crate::pieces::common_moves;
use crate::consts::board_consts::K_INDEX;
use crate::consts::board_consts::*;
use crate::pieces::king;

pub fn possible_q(b: &Board, white: bool) -> Vec<Move> {
    let index = if white { 1 } else { 0 };
    let own = if white { b.white_pieces } else { b.black_pieces};
    let opp = if white { b.black_pieces - b.pieces[K_INDEX as usize] } else { b.white_pieces - b.pieces[(K_INDEX + 1) as usize]};
    let mut list: Vec<Move> = Vec::new();

    if king::is_double_check(b.attackers) {
        return list;
    }
    let queens = b.pieces[(Q_INDEX + index) as usize];
    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & queens != 0 {
            let moves = b.get_pinned_slide(i) & !own & (common_moves::d_and_anti_d_moves(i, opp, own)
                | common_moves::h_and_vmoves(i, opp, own));
            for i2 in 0u8..64u8 {
                if 2u64.pow(i2 as u32) & moves &b.push_mask != 0 {
                    list.push(
                        Move::new_move(i, i2, opp & 2_u64.pow(i2 as u32) != 0)
                    );
                }
            }
        }
    }
    return list;
}

pub fn watched_by_q(b: &Board, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let own = if white { b.white_pieces } else { b.black_pieces };
    let opp = if white { b.black_pieces - b.pieces[K_INDEX as usize] } else { b.white_pieces - b.pieces[(K_INDEX + 1) as usize] };
    let mut moves = 0;

    let queens = b.pieces[(Q_INDEX + index) as usize];
    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & queens != 0 {
            moves |= (common_moves::d_and_anti_d_moves(i, opp, own)
                | common_moves::h_and_vmoves(i, opp, own))
                & b.get_pinned_slide(i);
        }
    }
    return moves;
}

pub fn attacked_from_square(b: &Board, square: u8, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let own = if white { b.white_pieces } else { b.black_pieces };
    let opp = if white { b.black_pieces } else { b.white_pieces };
    let moves = common_moves::h_and_vmoves(square, opp - b.pieces[(K_INDEX + index) as usize], own)
        | common_moves::d_and_anti_d_moves(square, opp - b.pieces[(K_INDEX + index) as usize], own);

    return moves;
}
