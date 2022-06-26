use crate::{Board, Move, print_u64_bitboard};
use crate::pieces::common_moves;
use crate::consts::board_consts::*;
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
    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & rooks != 0 {
            let moves = !own & common_moves::h_and_vmoves(i, opp, own);
            for i2 in 0u8..64u8 {
                if 2u64.pow(i2 as u32) & moves != 0 {
                    list.push(
                        Move::new_move(i, i2, opp & 2_u64.pow(i2 as u32) != 0)
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
        if 2_u64.pow(i as u32) & rooks != 0 {
            moves |= common_moves::h_and_vmoves(i, opp, own);
        }
    }
    return moves;
}
pub fn attacked_from_square(b: &Board, square: u8, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let own = if white { b.white_pieces } else { b.black_pieces };
    let moves = common_moves::h_and_vmoves(square, own - b.pieces[(K_INDEX + index) as usize], own);

    return moves;
}
