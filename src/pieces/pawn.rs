use crate::game::FILE_MASKS;
use crate::pieces::{common_moves, king};
use crate::{Board, Move, pieces, print_u64_bitboard};
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
        // todo det går kanskje ann å en passante når man bare skal ta vanlig?
        // en passant
        pawn_moves = (b.push_mask << 8) & (((b.pieces[(P_INDEX + index) as usize] << 9) & (opposing_pieces << 8) & (RANK_MASKS[5]) & (!FILE_MASKS[0])) & if Move::last_move_was_double_push(b.last_move) { (1 << (b.last_move.from & MOVE_MASK) as u32) >> 8 } else { 0 });  // capture right
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 9) & ((1u64 << i))) != 0 && check_ep_legal(b, 1 << (i - 9), 1 << (i - 8), true) {
                list.push(Move::new_ep(i - 9, i));
            }
        }

        pawn_moves = (b.push_mask << 8) & (((b.pieces[(P_INDEX + index) as usize] << 7) & (opposing_pieces << 8) & (RANK_MASKS[5]) & (!FILE_MASKS[0])) & if Move::last_move_was_double_push(b.last_move) { (1 << (b.last_move.from & MOVE_MASK) as u32) >> 8 } else { 0 }); // capture left
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i - 7) & ((1u64 << i))) != 0 && check_ep_legal(b, 1 << (i - 7), 1 << (i - 8), true) {
                println!("ep2");
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
                list.push(Move::new_double_push(i + 16, i));
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
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 9) & ((1u64 << i))) != 0 && check_ep_legal(b, 1 << (i + 9), 1 << (i + 8), false) {
                list.push(Move::new_ep(i + 9, i));
            }
        }

        pawn_moves = (b.push_mask >> 8) & (((b.pieces[(P_INDEX + index) as usize] >> 7) & (opposing_pieces >> 8) & (RANK_MASKS[2]) & (!FILE_MASKS[0])) & if Move::last_move_was_double_push(b.last_move) { ((1 << (b.last_move.from & MOVE_MASK)) as u64) << 8 } else { 0 }); // capture left
        for i in 0..64 {
            if (((pawn_moves >> i) & 1) == 1) && (b.get_pinned_slide(i + 7) & ((1u64 << i))) != 0 && check_ep_legal(b, 1 << (i + 7), 1 << (i + 8), false) {
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

fn check_ep_legal(b: &Board, move_piece: u64, taken_piece: u64, white: bool) -> bool {
    let index = if white { 1 } else { 0 };
    if b.pieces[(K_INDEX + index) as usize] == 0 { return true };
    let opp = if white { b.black_pieces - taken_piece } else { b.white_pieces - taken_piece };
    let own = if white { b.white_pieces - move_piece } else { b.black_pieces - move_piece };
    let king_square: u8 = (63 - b.pieces[(K_INDEX + index) as usize].leading_zeros()) as u8;

    let d_moves = common_moves::d_and_anti_d_moves(king_square, opp - b.pieces[(K_INDEX + 1 - index) as usize], own);
    let line_moves = common_moves::h_and_vmoves(king_square, opp - b.pieces[(K_INDEX + 1 - index) as usize], own);
    let attackers =
        d_moves & b.pieces[(B_INDEX + 1 - index) as usize]
            | (d_moves | line_moves) & b.pieces[(Q_INDEX + 1 - index) as usize]
            | line_moves & b.pieces[(R_INDEX + 1 - index) as usize];
    return attackers == 0;
}




