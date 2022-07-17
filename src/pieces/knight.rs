use num_format::Locale::el;
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
    let opposing_pieces = if white { b.black_pieces } else { b.white_pieces };
    let own_pieces = if !white { b.black_pieces } else { b.white_pieces };
    let index = if white { 1 } else { 0 };

    let mut list: Vec<Move> = Vec::new();
    if king::is_double_check(b.attackers) {
        return list;
    }


    let knights = b.pieces[(N_INDEX + index) as usize];



    for i in 0..64 {
        if (1 << i) & knights != 0 {

            let moves =
                KNIGHT_MOVES[i] & !own_pieces & b.push_mask & b.get_pinned_slide(i as u8);

            for i2 in 0..64 {
                if (1 << i2) & moves != 0 {
                    list.push(
                        Move::new_move(
                            i as u8,
                            i2,
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
    let index = if white { 1 } else { 0 };
    let knights = b.pieces[(N_INDEX + index) as usize];
    for i in 0..64 {
        if (1 << i) & knights != 0 {
            return KNIGHT_MOVES[i];
        }
    }
    return 0;
}

pub fn attacked_from(square: u8) -> u64 {
    return KNIGHT_MOVES[square as usize];
}