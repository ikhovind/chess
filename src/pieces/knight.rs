use crate::{Board, Move, print_u64_bitboard};
use crate::game::FILE_MASKS;
use crate::pieces::bishop::watched_by_b;
use crate::pieces::pawn::watched_by_p;
use crate::pieces::queen::watched_by_q;
use crate::pieces::rook::watched_by_r;

pub fn possible_n(b: &Board, white: bool) -> Vec<Move> {
    let mut opposing_pieces: u64 = b.white_pieces;
    let mut own_pieces = b.black_pieces;
    let mut index = 0;
    if white {
        opposing_pieces = b.black_pieces;
        own_pieces = b.white_pieces;
        index = 1;
    }
    let mut list: Vec<Move> = Vec::new();

    let knights = b.knights[index];
    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & knights != 0 {
            let spot_1_clip = !(FILE_MASKS[0] & FILE_MASKS[1]);
            let spot_2_clip = !FILE_MASKS[0];
            let spot_3_clip = !FILE_MASKS[7];
            let spot_4_clip = !(FILE_MASKS[7] & FILE_MASKS[6]);

            let spot_5_clip = !(FILE_MASKS[7] & FILE_MASKS[6]);
            let spot_6_clip = !FILE_MASKS[7];
            let spot_7_clip = !FILE_MASKS[0];
            let spot_8_clip = !(FILE_MASKS[0] & FILE_MASKS[1]);

            /* The clipping masks we just created will be used to ensure that no
        under or overflow positions are computed when calculating the
        possible moves of the knight in certain files. */

            let spot_1 = ((1 << i)  & spot_1_clip) << 6;
            let spot_2 = ((1 << i) & spot_2_clip) << 15;
            let spot_3 = ((1 << i) & spot_3_clip) << 17;
            let spot_4 = ((1 << i)  & spot_4_clip) << 10;

            let spot_5 = ((1 << i)  & spot_5_clip) >> 6;
            let spot_6 = ((1 << i) & spot_6_clip) >> 15;
            let spot_7 = ((1 << i) & spot_7_clip) >> 17;
            let spot_8 = ((1 << i) & spot_8_clip) >> 10;

            let moves =
                (spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8)
                    & !own_pieces;
            for i2 in 0u8..64u8 {
                if 2u64.pow(i2 as u32) & moves != 0 {
                    list.push(
                        Move::new_move(
                            i,
                            i2,
                            opposing_pieces & 2_u64.pow(i2 as u32) != 0
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


pub fn watched_by_n(b: &Board, white:bool) -> u64 {
    let mut index = 0;
    if white {
        index = 1;
    }
    let mut moves = 0;
    let knights = b.knights[index];
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