use crate::{Board, Move};

pub fn watched_by_b(b: &Board, white: bool) -> u64 {
    let mut index = 0;
    let mut moves = 0;
    if white {
        index = 1;
    }
    let bishops = b.bishops[index];
    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & bishops != 0 {
            moves |= b.d_and_anti_d_moves(i as usize, white);
        }
    }
    return moves;
}

pub fn possible_b(b: &Board, white: bool) -> Vec<Move> {
    let mut opposing_pieces: u64 = b.white_pieces;
    let mut own_pieces = b.black_pieces;
    let mut index = 0;
    if white {
        opposing_pieces = b.black_pieces;
        own_pieces = b.white_pieces;
        index = 1;
    }
    let mut list: Vec<Move> = Vec::new();
    let bishops = b.bishops[index];
    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & bishops != 0 {
            let moves = b.d_and_anti_d_moves(i as usize, white) & !(own_pieces);
            for i2 in 0u8..64u8 {
                if 2u64.pow(i2 as u32) & moves != 0 {
                    list.push(
                        Move::new_move(i, i2, opposing_pieces & 2_u64.pow(i2 as u32) != 0)
                    );
                }
            }
        }
    }
    return list;
}
