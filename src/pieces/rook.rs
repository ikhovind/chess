use crate::{Board, Move};
use crate::pieces::common_moves;

pub fn possible_r(b: &Board, white: bool) -> Vec<Move> {
    let index = if white { 1 } else { 0 };
    let own = if white { b.white_pieces } else { b.black_pieces };
    let opp = if white { b.black_pieces - b.kings[0] } else { b.white_pieces - b.kings[1] };
    let mut list: Vec<Move> = Vec::new();
    let rooks = b.rooks[index];

    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & rooks != 0 {
            let moves = common_moves::h_and_vmoves(i, opp, own);
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
    let opp = if white { b.black_pieces - b.kings[0] } else { b.white_pieces - b.kings[1] };
    let mut moves = 0;

    let rooks = b.rooks[index];
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
    let moves = common_moves::h_and_vmoves(square, own - b.kings[index], own);

    return moves;
}