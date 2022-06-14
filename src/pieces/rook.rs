use crate::{Board, Move};

pub fn possible_r(b: &Board, white: bool) -> Vec<Move> {
    let mut opposing_pieces: u64 = b.white_pieces;
    let mut own_pieces = b.black_pieces;
    let mut index = 0;
    if white {
        opposing_pieces = b.black_pieces;
        own_pieces = b.white_pieces;
        index = 1;
    }
    let mut list: Vec<Move> = Vec::new();
    let rooks = b.rooks[index];
    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & rooks != 0 {
            let moves = b.h_and_vmoves(i as usize, white) & !(own_pieces);
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

pub fn watched_by_r(b: &Board, white: bool) -> u64 {
    let mut index = 0;
    let mut moves = 0;
    if white {
        index = 1;
    }
    let rooks = b.rooks[index];
    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & rooks != 0 {
            moves |= b.h_and_vmoves(i as usize, white);
        }
    }
    return moves;
}
