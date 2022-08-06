use crate::{Board};
use crate::consts::board_consts::*;
use crate::mv::Move;
use crate::move_gen::pieces::{common_moves};

pub fn possible_p(b: &Board, white: bool, captures: bool) -> Vec<Move> {
    let mut list: Vec<Move> = Vec::new();
    let index = if white { 1 } else { 0 };

    let opposing_pieces = if white { b.get_black_pieces() } else { b.get_white_pieces() };
    if white {
        let mut pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 8) & b.get_empty() & !RANK_MASKS[7]);//move 1 forward
        if !captures {
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & b.get_pinned_slide(i - 8) & (1u64 << i) != 0 {
                    list.push(Move::new_move(i - 8, i, false));
                }
            }
            pawn_moves = b.push_mask & (((b.pieces[(P_INDEX + index) as usize] << 16) & (b.get_empty() & (b.get_empty() << 8))) & RANK_MASKS[3]);//move 2 forward
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & &b.get_pinned_slide(i - 16) & (1u64 << i) != 0 {
                    list.push(Move::new_double_push(i - 16, i));
                }
            }

            pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] << 8) & b.get_empty() & RANK_MASKS[7];//pawn promotion by move 1 forward
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & b.get_pinned_slide(i - 8) & (1u64 << i) != 0 {
                    list.push(Move::new_promotion(i - 8, i, false, QUEEN));
                    list.push(Move::new_promotion(i - 8, i, false, ROOK));
                    list.push(Move::new_promotion(i - 8, i, false, BISHOP));
                    list.push(Move::new_promotion(i - 8, i, false, KNIGHT));
                }
            }
        }

        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 9) & (opposing_pieces) & !(RANK_MASKS[7] | FILE_MASKS[0])); // capture right
        for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
            if pawn_moves & b.get_pinned_slide(i - 9) & (1u64 << i) != 0 {
                list.push(Move::new_move(i - 9, i, true));
            }
        }

        pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] << 7) & (opposing_pieces) & !(RANK_MASKS[7] | FILE_MASKS[7]); // capture left
        for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
            if pawn_moves & b.get_pinned_slide(i - 7) & (1u64 << i) != 0 {
                list.push(Move::new_move(i - 7, i, true));
            }
        }
        //y1,y2,Promotion Type,"P"
        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 7) & opposing_pieces & RANK_MASKS[7] & !FILE_MASKS[7]);//pawn promotion by capture left
        for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
            if pawn_moves & b.get_pinned_slide(i - 7) & (1u64 << i) != 0 {
                list.push(Move::new_promotion(i - 7, i, true, QUEEN));
                list.push(Move::new_promotion(i - 7, i, true, ROOK));
                list.push(Move::new_promotion(i - 7, i, true, BISHOP));
                list.push(Move::new_promotion(i - 7, i, true, KNIGHT));
            }
        }

        pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] << 9) & opposing_pieces & RANK_MASKS[7] & !FILE_MASKS[0];//pawn promotion by capture right
        for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
            if pawn_moves & b.get_pinned_slide(i - 9) & (1u64 << i) != 0 {
                list.push(Move::new_promotion(i - 9, i, true, QUEEN));
                list.push(Move::new_promotion(i - 9, i, true, ROOK));
                list.push(Move::new_promotion(i - 9, i, true, BISHOP));
                list.push(Move::new_promotion(i - 9, i, true, KNIGHT));
            }
        }

        // en passant
        if Move::last_move_was_double_push(b.last_move) {
            pawn_moves = (b.push_mask << 8) & (b.pieces[(P_INDEX + index) as usize] << 9) & (opposing_pieces << 8) & RANK_MASKS[5] & !FILE_MASKS[0] & ((1 << (b.last_move.from & MOVE_MASK) as u32) >> 8);  // capture right
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & b.get_pinned_slide(i - 9) & (1u64 << i) != 0 && check_ep_legal(b, 1 << (i - 9), 1 << (i - 8), true) {
                    list.push(Move::new_ep(i - 9, i));
                    break;
                }
            }

            pawn_moves = (b.push_mask << 8) & (b.pieces[(P_INDEX + index) as usize] << 7) & (opposing_pieces << 8) & (RANK_MASKS[5]) & (!FILE_MASKS[7]) & ((1 << (b.last_move.from & MOVE_MASK) as u32) >> 8); // capture left
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & b.get_pinned_slide(i - 7) & (1u64 << i) != 0 && check_ep_legal(b, 1 << (i - 7), 1 << (i - 8), true) {
                    list.push(Move::new_ep(i - 7, i));
                    break;
                }
            }
        }
    } else {
        let mut pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] >> 8) & b.get_empty() & !RANK_MASKS[0];//move 1 forward
        if !captures {
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & b.get_pinned_slide(i + 8) & (1u64 << i) != 0 {
                    list.push(Move::new_move(i + 8, i, false));
                }
            }
            pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] >> 16) & b.get_empty() & (b.get_empty() >> 8) & RANK_MASKS[4];//move 2 forward
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & (1 << i) & b.get_pinned_slide(i + 16) != 0 {
                    list.push(Move::new_double_push(i + 16, i));
                }
            }

            pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] >> 8) & b.get_empty() & RANK_MASKS[0];//pawn promotion by move 1 forward
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & b.get_pinned_slide(i + 8) & (1u64 << i) != 0 {
                    list.push(Move::new_promotion(i + 8, i, false, QUEEN));
                    list.push(Move::new_promotion(i + 8, i, false, ROOK));
                    list.push(Move::new_promotion(i + 8, i, false, BISHOP));
                    list.push(Move::new_promotion(i + 8, i, false, KNIGHT));
                }
            }
        }
        pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] >> 7) & (opposing_pieces) & !(RANK_MASKS[0] | FILE_MASKS[0]); // capture left
        for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
            if pawn_moves & b.get_pinned_slide(i + 7) & (1u64 << i) != 0 {
                list.push(Move::new_move(i + 7, i, true));
            }
        }

        pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] >> 9) & (opposing_pieces) & !(RANK_MASKS[0] | FILE_MASKS[7]); // capture right
        for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
            if pawn_moves & b.get_pinned_slide(i + 9) & (1u64 << i) != 0 {
                list.push(Move::new_move(i + 9, i, true));
            }
        }
        //y1,y2,Promotion Type,"P"
        pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] >> 9) & opposing_pieces & RANK_MASKS[0] & !FILE_MASKS[7];//pawn promotion by capture right
        for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
            if pawn_moves & b.get_pinned_slide(i + 9) & (1u64 << i) != 0 {
                list.push(Move::new_promotion(i + 9, i, true, QUEEN));
                list.push(Move::new_promotion(i + 9, i, true, ROOK));
                list.push(Move::new_promotion(i + 9, i, true, BISHOP));
                list.push(Move::new_promotion(i + 9, i, true, KNIGHT));
            }
        }

        pawn_moves = b.push_mask & (b.pieces[(P_INDEX + index) as usize] >> 7) & opposing_pieces & RANK_MASKS[0] & !FILE_MASKS[0];//pawn promotion by capture left
        for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
            if pawn_moves & b.get_pinned_slide(i + 7) & (1u64 << i) != 0 {
                list.push(Move::new_promotion(i + 7, i, true, QUEEN));
                list.push(Move::new_promotion(i + 7, i, true, ROOK));
                list.push(Move::new_promotion(i + 7, i, true, BISHOP));
                list.push(Move::new_promotion(i + 7, i, true, KNIGHT));
            }
        }


        if Move::last_move_was_double_push(b.last_move) {
            pawn_moves = (b.push_mask >> 8) & (b.pieces[(P_INDEX + index) as usize] >> 9) & (opposing_pieces >> 8) & (RANK_MASKS[2]) & (!FILE_MASKS[7]) & if Move::last_move_was_double_push(b.last_move) { ((1 << (b.last_move.from & MOVE_MASK)) as u64) << 8 } else { 0 };  // capture right
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & b.get_pinned_slide(i + 9) & (1u64 << i) != 0 && check_ep_legal(b, 1 << (i + 9), 1 << (i + 8), false) {
                    list.push(Move::new_ep(i + 9, i));
                }
            }

            pawn_moves = (b.push_mask >> 8) & (b.pieces[(P_INDEX + index) as usize] >> 7) & (opposing_pieces >> 8) & (RANK_MASKS[2]) & (!FILE_MASKS[0]) & if Move::last_move_was_double_push(b.last_move) { ((1 << (b.last_move.from & MOVE_MASK)) as u64) << 8 } else { 0 }; // capture left
            for i in (pawn_moves.trailing_zeros() as u8)..(64u8 - pawn_moves.leading_zeros() as u8) {
                if pawn_moves & b.get_pinned_slide(i + 7) & (1u64 << i) != 0 && check_ep_legal(b, 1 << (i + 7), 1 << (i + 8), false) {
                    list.push(Move::new_ep(i + 7, i));
                }
            }
        }
    }
    list
}

#[inline(always)]
pub fn watched_by_p(b: &Board, white: bool) -> u64 {
    return if white {
        (b.pieces[P_INDEX + 1] << 9) & (!FILE_MASKS[0]) | (b.pieces[P_INDEX + 1] << 7) & (!FILE_MASKS[7]) // capture left
    } else {
        (b.pieces[P_INDEX] >> 9) & (!FILE_MASKS[7]) | (b.pieces[P_INDEX] >> 7) & (!FILE_MASKS[0]) // capture left
    }
}

pub fn attacked_from_square(square: u8, white: bool) -> u64 {
    let s = 1 << square;
    return if white {
        (s >> 9) & (!FILE_MASKS[7]) | (s >> 7) & (!FILE_MASKS[0]) // capture left
    } else {
        (s << 9) & (!FILE_MASKS[0]) | (s << 7) & (!FILE_MASKS[7]) // capture left
    }
}

fn check_ep_legal(b: &Board, move_piece: u64, taken_piece: u64, white: bool) -> bool {
    let index = if white { 1 } else { 0 };
    if b.pieces[(K_INDEX + index) as usize] == 0 { return true; };
    let opp = if white { b.get_black_pieces() - taken_piece } else { b.get_white_pieces() - taken_piece };
    let own = if white { b.get_white_pieces() - move_piece } else { b.get_black_pieces() - move_piece };
    let king_square: u8 = b.pieces[(K_INDEX + index) as usize].trailing_zeros() as u8;

    let d_moves = common_moves::d_and_anti_d_moves(king_square, opp, own);
    let line_moves = common_moves::h_and_vmoves(king_square, opp, own) & RANK_MASKS[(king_square / 8) as usize];
    let attackers =
        d_moves & b.pieces[(B_INDEX + 1 - index) as usize]
            | (d_moves | line_moves) & b.pieces[(Q_INDEX + 1 - index) as usize]
            | line_moves & b.pieces[(R_INDEX + 1 - index) as usize];
    attackers == 0
}




