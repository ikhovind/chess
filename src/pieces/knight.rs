use crate::{Board, computed, print_u64_bitboard};
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
    if king::is_double_check(b.attackers) {
        return list;
    }


    let knights = b.pieces[(N_INDEX + index) as usize];



    for i in 0..64 {
        if (1 << i) & knights != 0 {

            let moves =
            computed::lookup_consts::KNIGHT_MOVES[i]
                    & !own_pieces & b.push_mask & b.get_pinned_slide(i as u8);

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
    let mut index = 0;
    if white {
        index = 1;
    }
    let mut moves = 0;
    let knights = b.pieces[(N_INDEX + index) as usize];
    for i in 0u8..64u8 {
        if (1 << i) & knights != 0 {
            let spot_1_clip = !FILE_MASKS[0] & !FILE_MASKS[1];
            let spot_2_clip = !FILE_MASKS[0];
            let spot_3_clip = !FILE_MASKS[7];
            let spot_4_clip = !FILE_MASKS[7] & !FILE_MASKS[6];

            let spot_5_clip = !FILE_MASKS[7] & !FILE_MASKS[6];
            let spot_6_clip = !FILE_MASKS[7];
            let spot_7_clip = !FILE_MASKS[0];
            let spot_8_clip = !FILE_MASKS[0] & !FILE_MASKS[1];

            /* The clipping masks we just created will be used to ensure that no
        under or overflow positions are computed when calculating the
        possible moves of the knight in certain files. */

            let spot_1 = ((1 << i) & spot_1_clip) << 6;
            let spot_2 = ((1 << i) & spot_2_clip) << 15;
            let spot_3 = ((1 << i) & spot_3_clip) << 17;
            let spot_4 = ((1 << i) & spot_4_clip) << 10;

            let spot_5 = ((1 << i) & spot_5_clip) >> 6;
            let spot_6 = ((1 << i) & spot_6_clip) >> 15;
            let spot_7 = ((1 << i) & spot_7_clip) >> 17;
            let spot_8 = ((1 << i) & spot_8_clip) >> 10;

            moves = moves | spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;
        }
    }
    return moves;
}

pub fn attacked_from(square: u8) -> u64 {
    let spot_1_clip = !FILE_MASKS[0] & !FILE_MASKS[1];
    let spot_2_clip = !FILE_MASKS[0];
    let spot_3_clip = !FILE_MASKS[7];
    let spot_4_clip = !FILE_MASKS[7] & !FILE_MASKS[6];

    let spot_5_clip = !FILE_MASKS[7] & !FILE_MASKS[6];
    let spot_6_clip = !FILE_MASKS[7];
    let spot_7_clip = !FILE_MASKS[0];
    let spot_8_clip = !FILE_MASKS[0] & !FILE_MASKS[1];

    /* The clipping masks we just created will be used to ensure that no
under or overflow positions are computed when calculating the
possible moves of the knight in certain files. */

    let spot_1 = ((1u64 << square) & spot_1_clip) << 6;
    let spot_2 = ((1u64 << square) & spot_2_clip) << 15;
    let spot_3 = ((1u64 << square) & spot_3_clip) << 17;
    let spot_4 = ((1u64 << square) & spot_4_clip) << 10;

    let spot_5 = ((1u64 << square) & spot_5_clip) >> 6;
    let spot_6 = ((1u64 << square) & spot_6_clip) >> 15;
    let spot_7 = ((1u64 << square) & spot_7_clip) >> 17;
    let spot_8 = ((1u64 << square) & spot_8_clip) >> 10;

    return spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;
}