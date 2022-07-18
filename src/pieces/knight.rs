use crate::{Board, computed, print_u64_bitboard};
use crate::computed::lookup_consts::KNIGHT_MOVES;
use crate::consts::board_consts::*;
use crate::mv::Move;
use crate::pieces::bishop::watched_by_b;
use crate::pieces::king;
use crate::pieces::pawn::watched_by_p;
use crate::pieces::queen::watched_by_q;
use crate::pieces::rook::watched_by_r;

pub fn possible_n(b: &Board, white: bool) -> Vec<Move> {
    let opposing_pieces = if white { b.get_black_pieces() } else { b.get_white_pieces() };
    let own_pieces = if !white { b.get_black_pieces() } else { b.get_white_pieces() };
    let index = if white { 1 } else { 0 };

    let mut list: Vec<Move> = Vec::new();


    let knights = b.pieces[(N_INDEX + index) as usize];



    for i in (knights.trailing_zeros())..(64 - knights.leading_zeros()) {
        if (1 << i) & knights != 0 {

            let moves =
            computed::lookup_consts::KNIGHT_MOVES[i as usize]
                    & !own_pieces & b.push_mask & b.get_pinned_slide(i as u8);

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