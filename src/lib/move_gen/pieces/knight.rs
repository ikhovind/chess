use crate::Board;
use crate::consts::board_consts::*;
use crate::move_gen::computed_moves;
use crate::move_gen::computed_moves::lookup_consts::KNIGHT_MOVES;
use crate::move_gen::pieces::common_moves::add_moves_to_list;
use crate::mv::Move;

pub fn possible_n(b: &Board, captures: bool) -> Vec<Move> {
    let opposing_pieces = if b.white_turn { b.get_black_pieces() } else { b.get_white_pieces() };
    let cap_mask = if captures { opposing_pieces } else { u64::MAX };
    let own_pieces = if !b.white_turn { b.get_black_pieces() } else { b.get_white_pieces() };
    let index = if b.white_turn { 1 } else { 0 };



    let knights = b.pieces[(N_INDEX + index) as usize];


    let mut list: Vec<Move> = Vec::with_capacity((knights.count_ones() * 6) as usize);

    for i in (knights.trailing_zeros())..(64 - knights.leading_zeros()) {
        if (1 << i) & knights != 0 {

            let moves =
            computed_moves::lookup_consts::KNIGHT_MOVES[i as usize]
                    & !own_pieces & b.push_mask & b.get_pinned_slide(i as u8) & cap_mask;

            add_moves_to_list(opposing_pieces, &mut list, i, moves);
        }
    }

    /* compute only the places where the knight can move and attack. The
        caller will determine if this is a white or black night. */
    list
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
    moves
}

pub fn attacked_from(square: u8) -> u64 {
    KNIGHT_MOVES[square as usize]
}