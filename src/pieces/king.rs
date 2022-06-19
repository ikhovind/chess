use crate::{Board, Move, pieces, print_u64_bitboard};
use crate::game::FILE_MASKS;
use crate::pieces::bishop::watched_by_b;
use crate::pieces::knight::watched_by_n;
use crate::pieces::rook::watched_by_r;

pub fn possible_k(b: &Board, white: bool) -> Vec<Move> {
    let mut opposing_pieces: u64 = b.white_pieces;
    let mut opponent_watching: u64 = b.watched_squares_white;
    let mut own_pieces = b.black_pieces;
    let mut index = 0;
    let mut short_castle= b.black_short_c;
    let mut long_castle = b.black_long_c;
    let mut short_castle_sq = vec![
        1 << 60,
        1 << 61,
        1 << 62,
    ];
    let mut long_castle_sq = vec![
        1 << 58,
        1 << 59,
        1 << 60,
    ];
    if white {
        opposing_pieces = b.black_pieces;
        opponent_watching = b.watched_squares_black;
        own_pieces = b.white_pieces;
        index = 1;
        short_castle = b.white_short_c;
        long_castle = b.white_long_c;

        long_castle_sq = vec![
            1 << 1,
            1 << 2,
            1 << 3,
            1 << 4
        ];
        short_castle_sq = vec![
            1 << 4,
            1 << 5,
            1 << 6,
        ];
    }
    let mut list: Vec<Move> = Vec::new();
    let kings = b.kings[index];

    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & kings != 0 {
            let king_loc = 1 << i;
            let king_clip_file_h = king_loc & !FILE_MASKS[7];
            let king_clip_file_a = king_loc & !FILE_MASKS[0];

            /* remember the representation of the board in relation to the bitindex
                when looking at these shifts.... */
            let spot_1 = king_clip_file_h << 7;
            let spot_2 = king_loc << 8;
            let spot_3 = king_clip_file_h << 9;
            let spot_4 = king_clip_file_h << 1;

            let spot_5 = king_clip_file_a >> 7;
            let spot_6 = king_loc >> 8;
            let spot_7 = king_clip_file_a >> 9;
            let spot_8 = king_clip_file_a >> 1;

            let moves = (spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 |
                spot_7 | spot_8) & !own_pieces & !opponent_watching;

            if long_castle && !long_castle_sq.iter().any(|&x| (x & ((b.white_pieces - b.kings[1]) | (b.black_pieces - b.kings[0])) != 0)) {
                list.push(Move::new_castle(if white { 4 } else { 60 }, if white { 2 } else { 58 }));
            }

            if short_castle && !short_castle_sq.iter().any(|&x| (x & ((b.white_pieces - b.kings[1]) | (b.black_pieces - b.kings[0])) != 0)) {
                list.push(Move::new_castle(if white { 4 } else { 60 }, if white {6} else { 62 }));
            }

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

pub fn watched_by_k(b: &Board, white: bool) -> u64 {
    let mut index = 0;
    if white {
        index = 1;
    }
    let mut moves = 0;
    let kings = b.kings[index];

    for i in 0u8..64u8 {
        if 2_u64.pow(i as u32) & kings != 0 {
            let king_loc = 1 << i;
            let king_clip_file_h = king_loc & !FILE_MASKS[7];
            let king_clip_file_a = king_loc & !FILE_MASKS[0];

            /* remember the representation of the board in relation to the bitindex
                when looking at these shifts.... */
            let spot_1 = king_clip_file_h << 7;
            let spot_2 = king_loc << 8;
            let spot_3 = king_clip_file_h << 9;
            let spot_4 = king_clip_file_h << 1;

            let spot_5 = king_clip_file_a >> 7;
            let spot_6 = king_loc >> 8;
            let spot_7 = king_clip_file_a >> 9;
            let spot_8 = king_clip_file_a >> 1;

            moves = moves | spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 |
                spot_7 | spot_8;
        }
    }
    return moves;
}

pub fn get_attackers(b: &Board, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let king_square: u8 = (63 - b.kings[index].leading_zeros()) as u8;
    let attackers =
        pieces::knight::attacked_from(king_square) & b.knights[1 - index]
        | pieces::bishop::attacked_from_square(b, king_square, !white) & b.bishops[1 - index]
        | pieces::pawn::attacked_from_square(king_square, !white) & b.pawns[1 - index]
        | pieces::queen::attacked_from_square(b, king_square, !white) & b.pawns[1 - index]
        | pieces::rook::attacked_from_square(b, king_square, !white) & b.pawns[1 - index];

    return attackers;
}

pub fn is_double_check(attackers: u64) -> bool {
    return attackers.leading_zeros() + attackers.trailing_zeros() + 1 < 63;
}