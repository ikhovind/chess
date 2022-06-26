#[cfg(test)]
use crate::{Board, Move, print_u64_bitboard};
use crate::pieces::*;
use crate::consts::board_consts::*;
use crate::game::*;
use crate::pieces::king::get_attackers;
use crate::pieces::pawn::watched_by_p;

#[test]
fn sliding_moves() {
    let mut b = Board::from_fen(String::from("k7/8/8/8/3R4/8/8/K7"));
    assert_eq!(common_moves::h_and_vmoves(27, b.black_pieces, b.white_pieces),
               FILE_MASKS[3] - (1 << 27) + RANK_MASKS[3] - (1 << 27));
}

#[test]
fn move_gen() {
    let mut b  = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    assert_eq!(b.get_num_moves(1), 20);
    assert_eq!(b.get_num_moves(2), 400);
    assert_eq!(b.get_num_moves(3), 8902);
    //assert_eq!(get_num_moves(b,4), 197281);
}

#[test]
fn blocks_check() {
    let mut b  = Board::from_fen(String::from("1k6/3r4/8/5R2/8/3K4/8/8"));
    assert_eq!(b.get_num_moves(1), 7);
}
