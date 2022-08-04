use crate::Board;
use crate::consts::board_consts::*;
use crate::move_gen::computed_moves;
use crate::move_gen::computed_moves::lookup_consts::KNIGHT_MOVES;
use crate::mv::Move;

pub fn possible_n(b: &Board, white: bool, captures: bool) -> Vec<Move> {
    let opposing_pieces = if white { b.get_black_pieces() } else { b.get_white_pieces() };
    let cap_mask = if captures { opposing_pieces } else { u64::MAX };
    let own_pieces = if !white { b.get_black_pieces() } else { b.get_white_pieces() };
    let index = if white { 1 } else { 0 };

    let mut list: Vec<Move> = Vec::new();


    let knights = b.pieces[(N_INDEX + index) as usize];



    for i in (knights.trailing_zeros())..(64 - knights.leading_zeros()) {
        if (1 << i) & knights != 0 {

            let moves =
            computed_moves::lookup_consts::KNIGHT_MOVES[i as usize]
                    & !own_pieces & b.push_mask & b.get_pinned_slide(i as u8) & cap_mask;

            for i2 in (moves.trailing_zeros())..(64 - moves.leading_zeros()) {
                if (1 << i2) & moves != 0 {
                    list.push(
                        Move::new_move(
                            i as u8,
                            i2 as u8,
                            opposing_pieces & (1 << i2) != 0,
                        )
                    );
                }
            }
        }
    }

    /* compute only the places where the knight can move and attack. The
        caller will determine if this is a white or black night. */
    return list;
}


pub fn watched_by_n(b: &Board, white: bool) -> u64 {
    let mut index = 0;
    if white {
        index = 1;
    }
    let mut moves = 0;
    let knights = b.pieces[(N_INDEX + index) as usize];
    for i in (knights.trailing_zeros())..(64 - knights.leading_zeros()) {
        if (1 << i) & knights != 0 {
            moves |= KNIGHT_MOVES[i as usize];
        }
    }
    return moves;
}

pub fn attacked_from(square: u8) -> u64 {
    return KNIGHT_MOVES[square as usize];
}