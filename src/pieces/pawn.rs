use crate::game::FILE_MASKS;
use crate::pieces::king;
use crate::{Board, Move, print_u64_bitboard};
use crate::mv::{BISHOP, KNIGHT, QUEEN, RANK_MASKS, ROOK};
use crate::consts::board_consts::*;

pub fn possible_p(b: &Board, white: bool) -> Vec<Move> {
    let mut list: Vec<Move> = Vec::new();
    let index = if white { 1 } else { 0 };

    if king::is_double_check(b.attackers) {
        return list;
    }
    let opposing_pieces = if white { b.black_pieces } else { b.white_pieces };
    if white {
        let mut pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 9) & (opposing_pieces) & (!RANK_MASKS[7]) & (!FILE_MASKS[0])); // capture right
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 9) & ((1u64 << i))) != 0 {
                list.push(Move::new_move(i - 9, i, true));
            }
        }

        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 7) & (opposing_pieces) & (!RANK_MASKS[7]) & (!FILE_MASKS[7])); // capture left
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 7) & ((1u64 << i))) != 0 {
                list.push(Move::new_move(i - 7, i, true));
            }
        }
        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 8) & b.empty & !RANK_MASKS[7]);//move 1 forward
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 8) & ((1u64 << i))) != 0 {
                list.push(Move::new_move(i - 8, i, false));
            }
        }
        pawn_moves = b.push_mask & (((b.pieces[(P_INDEX + index) as usize] << 16) & (b.empty & (b.empty << 8))) & RANK_MASKS[3]);//move 2 forward
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 16) & ((1u64 << i))) != 0 {
                list.push(Move::new_move(i - 16, i, false));
            }
        }
        //y1,y2,Promotion Type,"P"
        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 7) & opposing_pieces & RANK_MASKS[7] & !FILE_MASKS[0]);//pawn promotion by capture left
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 7) & ((1u64 << i))) != 0 {
                list.push(Move::new_promotion(i - 7, i, true, QUEEN));
                list.push(Move::new_promotion(i - 7, i, true, ROOK));
                list.push(Move::new_promotion(i - 7, i, true, BISHOP));
                list.push(Move::new_promotion(i - 7, i, true, KNIGHT));
            }
        }

        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 9) & opposing_pieces & RANK_MASKS[7] & !FILE_MASKS[7]);//pawn promotion by capture right
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 9) & ((1u64 << i))) != 0 {
                list.push(Move::new_promotion(i - 9, i, true, QUEEN));
                list.push(Move::new_promotion(i - 9, i, true, ROOK));
                list.push(Move::new_promotion(i - 9, i, true, BISHOP));
                list.push(Move::new_promotion(i - 9, i, true, KNIGHT));
            }
        }

        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] << 8) & b.empty & RANK_MASKS[7]);//pawn promotion by move 1 forward
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 8) & ((1u64 << i))) != 0 {
                list.push(Move::new_promotion(i - 8, i, false, QUEEN));
                list.push(Move::new_promotion(i - 8, i, false, ROOK));
                list.push(Move::new_promotion(i - 8, i, false, BISHOP));
                list.push(Move::new_promotion(i - 8, i, false, KNIGHT));
            }
        }
        // en passant
        pawn_moves = (b.push_mask << 8) & (((b.pieces[(P_INDEX + index) as usize] << 9) & (opposing_pieces << 8) & (!RANK_MASKS[7]) & (!FILE_MASKS[7])) & if Move::last_move_was_double_push(b.last_move) { (1 << (b.last_move.from & MOVE_MASK) as u32) >> 8 } else { 0 });  // capture right
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 9) & ((1u64 << i))) != 0 {
                list.push(Move::new_ep(i - 9, i));
            }
        }

        pawn_moves = (b.push_mask << 8) & (((b.pieces[(P_INDEX + index) as usize] << 7) & (opposing_pieces << 8) & (!RANK_MASKS[7]) & (!FILE_MASKS[0])) & if Move::last_move_was_double_push(b.last_move) { (1 << (b.last_move.from & MOVE_MASK) as u32) >> 8 } else { 0 }); // capture left
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 7) & ((1u64 << i))) != 0 {
                list.push(Move::new_ep(i - 7, i));
            }
        }
    } else {
        let mut pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] >> 7) & (opposing_pieces) & (!RANK_MASKS[0]) & (!FILE_MASKS[0])); // capture left
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 7) & ((1u64 << i))) != 0 {
                list.push(Move::new_move(i + 7, i, true));
            }
        }

        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] >> 9) & (opposing_pieces) & (!RANK_MASKS[0]) & (!FILE_MASKS[7])); // capture right
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 9) & (1u64 << i)) != 0 {
                list.push(Move::new_move(i + 9, i, true));
            }
        }
        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] >> 8) & b.empty & !RANK_MASKS[0]);//move 1 forward
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 8) & ((1u64 << i))) != 0 {
                list.push(Move::new_move(i + 8, i, false));
            }
        }
        pawn_moves = b.push_mask & (((b.pieces[(P_INDEX + index) as usize] >> 16) & (b.empty & (b.empty >> 8))) & RANK_MASKS[4]);//move 2 forward
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 16) & ((1u64 << i))) != 0 {
                list.push(Move::new_move(i + 16, i, false));
            }
        }
        //y1,y2,Promotion Type,"P"
        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] >> 9) & opposing_pieces & RANK_MASKS[0] & !FILE_MASKS[7]);//pawn promotion by capture right
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 9) & ((1u64 << i))) != 0 {
                list.push(Move::new_promotion(i + 9, i, true, QUEEN));
                list.push(Move::new_promotion(i + 9, i, true, ROOK));
                list.push(Move::new_promotion(i + 9, i, true, BISHOP));
                list.push(Move::new_promotion(i + 9, i, true, KNIGHT));
            }
        }

        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] >> 7) & opposing_pieces & RANK_MASKS[0] & !FILE_MASKS[0]);//pawn promotion by capture left
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 7) & ((1u64 << i))) != 0 {
                list.push(Move::new_promotion(i + 7, i, true, QUEEN));
                list.push(Move::new_promotion(i + 7, i, true, ROOK));
                list.push(Move::new_promotion(i + 7, i, true, BISHOP));
                list.push(Move::new_promotion(i + 7, i, true, KNIGHT));
            }
        }

        pawn_moves = b.push_mask & ((b.pieces[(P_INDEX + index) as usize] >> 8) & b.empty & RANK_MASKS[0]);//pawn promotion by move 1 forward
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 8) & ((1u64 << i))) != 0 {
                list.push(Move::new_promotion(i + 8, i, false, QUEEN));
                list.push(Move::new_promotion(i + 8, i, false, ROOK));
                list.push(Move::new_promotion(i + 8, i, false, BISHOP));
                list.push(Move::new_promotion(i + 8, i, false, KNIGHT));
            }
        }

        pawn_moves = (b.push_mask >> 8) & (((b.pieces[(P_INDEX + index) as usize] >> 9) & (opposing_pieces >> 8) & (RANK_MASKS[2]) & (!FILE_MASKS[7])) & if Move::last_move_was_double_push(b.last_move) { ((1 << (b.last_move.from & MOVE_MASK)) as u64) << 8 } else { 0 });  // capture right
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 9) & ((1u64 << i))) != 0 {
                println!("pushing en passant: {}", i);
                list.push(Move::new_ep(i + 9, i));
            }
        }

        pawn_moves = (b.push_mask >> 8) & (((b.pieces[(P_INDEX + index) as usize] >> 7) & (opposing_pieces >> 8) & (RANK_MASKS[2]) & (!FILE_MASKS[0])) & if Move::last_move_was_double_push(b.last_move) { ((1 << (b.last_move.from & MOVE_MASK)) as u64) << 8 } else { 0 }); // capture left
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 7) & ((1u64 << i))) != 0 {
                list.push(Move::new_ep(i + 7, i));
            }
        }
    }
    return list;
}

pub fn watched_by_p(b: &Board, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let mut pawn_moves;
    if white {
        pawn_moves = (b.pieces[(P_INDEX + index) as usize] << 9) & (!FILE_MASKS[0]); // capture right
        pawn_moves |= ((b.pieces[(P_INDEX + index) as usize] << 7) & (!FILE_MASKS[7])); // capture left
    }
    else {
        pawn_moves = (b.pieces[(P_INDEX + index) as usize] >> 9) & (!FILE_MASKS[7]); // capture right
        pawn_moves = pawn_moves | ((b.pieces[(P_INDEX + index) as usize] >> 7) & (!FILE_MASKS[0])); // capture left
    }

    return pawn_moves;
}

pub fn attacked_from_square(square: u8, white: bool) -> u64 {
    let s = 1 << square;
    let mut pawn_moves;
    if white {
        pawn_moves = (s >> 9) & (!FILE_MASKS[7]); // capture right
        pawn_moves = pawn_moves | ((s >> 7) & (!FILE_MASKS[0])); // capture left
    } else {
        pawn_moves = (s << 9) & (!FILE_MASKS[7]); // capture right
        pawn_moves = pawn_moves | ((s << 7) & (!FILE_MASKS[0])); // capture left
    }

    return pawn_moves;
}


