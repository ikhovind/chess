use crate::{Board};
use crate::consts::board_consts::*;
use crate::move_gen::computed_moves::lookup_consts::KING_MOVES;
use crate::move_gen::pieces;
use crate::mv::Move;
use crate::move_gen::pieces::common_moves;
use crate::move_gen::pieces::common_moves::add_moves_to_list;

pub fn possible_k(b: &Board, captures: bool) -> Vec<Move> {
    let opposing_pieces: u64 = if b.white_turn {b.get_black_pieces()} else { b.get_white_pieces() };
    let opponent_watching: u64 = b.watched(!b.white_turn);
    let own_pieces = if b.white_turn { b.get_white_pieces() } else {b.get_black_pieces()};
    let index: u8 = if b.white_turn { 1 } else { 0 };
    let short_castle_sq = if b.white_turn {
        [
            1 << 5,
            1 << 6,
        ]
    } else {[
        1 << 61,
        1 << 62,
    ]};
    let long_castle_sq = if b.white_turn {
        [
            1 << 1,
            1 << 2,
            1 << 3,
        ]
    } else {[
        1 << 57,
        1 << 58,
        1 << 59,
    ]};

    let short_castle = b.castle_rights[(index * 2) as usize];
    let long_castle = b.castle_rights[(index * 2 + 1) as usize];
    let long_castle_rook = if b.white_turn { WHITE_LONG_ORG_ROOK } else { BLACK_LONG_ORG_ROOK };
    let mut list: Vec<Move> = Vec::with_capacity(5);
    let kings = b.pieces[K_INDEX + index as usize];
    let capture_mask = if captures { opposing_pieces } else { u64::MAX };

    for i in (kings.trailing_zeros())..(64 - kings.leading_zeros()) {
        if (1 << i) & kings != 0 {

            if !captures && b.push_mask == u64::MAX {
                // long castle
                if b.castle_rights[(index * 2 + 1) as usize] && (long_castle && !long_castle_sq.iter().any(|&x| (x & ((b.get_white_pieces()) | (b.get_black_pieces()) | (opponent_watching & (!(long_castle_rook << 1))))) != 0)) {
                    list.push(Move::new_castle(if b.white_turn { 4 } else { 60 }, if b.white_turn { 2 } else { 58 }));
                }

                // short castle
                if b.castle_rights[(index * 2) as usize] && (short_castle && !short_castle_sq.iter().any(|&x| (x & ((b.get_white_pieces()) | (b.get_black_pieces() - b.pieces[K_INDEX as usize]) | opponent_watching) != 0))) {
                    list.push(Move::new_castle(if b.white_turn { 4 } else { 60 }, if b.white_turn { 6 } else { 62 }));
                }
            }

            let moves = capture_mask & KING_MOVES[i as usize] & !own_pieces & !opponent_watching;
            add_moves_to_list(opposing_pieces, &mut list, i, moves);
        }
    }
    list
}

pub fn watched_by_k(b: &Board, white: bool) -> u64 {
    let mut index = 0;
    if white {
        index = 1;
    }
    let kings = b.pieces[(K_INDEX + index) as usize];

    for i in (kings.trailing_zeros())..(64 - kings.leading_zeros()) {
        if (1 << i) & kings != 0 {
            return KING_MOVES[i as usize];
        }
    }
    0
}

pub fn get_attackers(b: &Board, white: bool) -> u64 {
    let index = if white { 1 } else { 0 };
    let opp = if white { b.get_black_pieces() } else { b.get_white_pieces() };
    let own = if white { b.get_white_pieces() } else { b.get_black_pieces() };
    if b.pieces[(K_INDEX + index) as usize] == 0 { return 0; };
    let king_square: u8 = b.pieces[(K_INDEX + index) as usize].trailing_zeros() as u8;

    let d_moves = common_moves::d_and_anti_d_moves(king_square, opp, own);
    let line_moves = common_moves::h_and_vmoves(king_square, opp, own);
    
    d_moves & b.pieces[(B_INDEX + 1 - index) as usize]
            | (d_moves | line_moves) & b.pieces[(Q_INDEX + 1 - index) as usize]
            | line_moves & b.pieces[(R_INDEX + 1 - index) as usize]
            | pieces::pawn::attacked_from_square(king_square, !white) & b.pieces[(P_INDEX + 1 - index) as usize]
            | pieces::knight::attacked_from(king_square) & b.pieces[(N_INDEX + 1 - index) as usize]
}

pub fn is_double_check(attackers: u64) -> bool {
    attackers.leading_zeros() + attackers.trailing_zeros() + 1 < 63
}