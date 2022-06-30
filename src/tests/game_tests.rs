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
    assert_eq!(b.get_num_moves(4), 197281);
    assert_eq!(b.get_num_moves(5), 9531373);
}

#[test]
fn pos_5_perft() {
    // https://www.chessprogramming.org/Perft_Results
    let mut b  = Board::from_fen(String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R"));
    assert_eq!(b.get_num_moves(1), 44);
    assert_eq!(b.get_num_moves(2), 1486);
    assert_eq!(b.get_num_moves(3), 62379);
    assert_eq!(b.get_num_moves(4), 2103487);
    assert_eq!(b.get_num_moves(5),  89941194);
}

#[test]
fn blocks_check() {
    let mut b  = Board::from_fen(String::from("1k6/3r4/8/5R2/8/3K4/8/8"));

    assert_eq!(b.get_num_moves(1), 7);
}

#[test]
fn blocks_with_en_passant() {
    let mut b  = Board::from_fen(String::from("8/8/8/2k5/4p3/8/3P4/3K4"));
    b.make_move(Move::new_move(11, 27, false));
    assert_eq!(b.get_num_moves(1), 8);
}

#[test]
fn pinned_rook_can_slide() {
    let mut b  = Board::from_fen(String::from("4k3/8/4r3/8/8/4Q3/8/1K6"));
    b.make_move(Move::new_move(20, 28, false));
    assert_eq!(b.get_num_moves(1), 8);


    let mut b  = Board::from_fen(String::from("8/8/2k1r2Q/8/8/8/8/1K6"));
    b.make_move(Move::new_move(47, 46, false));
    assert_eq!(b.get_num_moves(1), 11);
}

#[test]
fn en_passant_discovered_check() {
    let mut b  = Board::from_fen(String::from("8/8/8/8/k2p3Q/8/2P5/2K5"));
    b.make_move(Move::new_move(10, 26, false));
    assert_eq!(b.get_num_moves(1), 5);
}
