#[cfg(test)]
use crate::{Board, print_u64_bitboard};
use crate::consts::board_consts::{FILE_MASKS, RANK_MASKS};
use crate::mv::Move;
use crate::pieces::common_moves;

#[test]
fn sliding_moves() {
    let mut b = Board::from_fen(String::from("k7/8/8/8/3R4/8/8/K7"));
    assert_eq!(common_moves::h_and_vmoves(27, b.black_pieces, b.white_pieces),
               FILE_MASKS[3] - (1 << 27) + RANK_MASKS[3] - (1 << 27));
}

#[test]
fn move_gen() {
    let mut b = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    assert_eq!(b.get_num_moves(1), 20);
    assert_eq!(b.get_num_moves(2), 400);
    assert_eq!(b.get_num_moves(3), 8902);
    assert_eq!(b.get_num_moves(4), 197281);
    assert_eq!(b.get_num_moves(5), 4865609);
    assert_eq!(b.get_num_moves(6), 119060324);
}

#[test]
fn pos_2_perft() {
    let mut b = Board::from_fen(String::from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - "));
    assert_eq!(b.get_num_moves(1), 48);
    assert_eq!(b.get_num_moves(2), 2039);
    assert_eq!(b.get_num_moves(3), 97862);
    assert_eq!(b.get_num_moves(4), 4085603);
    assert_eq!(b.get_num_moves(5), 193690690);
    assert_eq!(b.get_num_moves(6), 8031647685);
}

#[test]
fn pos_3_perft() {
    let mut b = Board::from_fen(String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - "));
    assert_eq!(b.get_num_moves(1), 14);
    assert_eq!(b.get_num_moves(2), 191);
    assert_eq!(b.get_num_moves(3), 2812);
    assert_eq!(b.get_num_moves(4), 43238);
    assert_eq!(b.get_num_moves(5), 674624);
    assert_eq!(b.get_num_moves(6), 11030083);
    assert_eq!(b.get_num_moves(7), 178633661);
    assert_eq!(b.get_num_moves(8), 3009794393);
}

#[test]
fn pos_4_perft() {
    let mut b = Board::from_fen(String::from("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"));
    assert_eq!(b.get_num_moves(1), 6);
    assert_eq!(b.get_num_moves(2), 264);
    assert_eq!(b.get_num_moves(3), 9467);
    assert_eq!(b.get_num_moves(4), 422333);
    assert_eq!(b.get_num_moves(5), 15833292);
    assert_eq!(b.get_num_moves(6), 706045033);
}

#[test]
fn pos_5_perft() {
    // https://www.chessprogramming.org/Perft_Results
    let mut b = Board::from_fen(String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 "));
    assert_eq!(b.get_num_moves(1), 44);
    assert_eq!(b.get_num_moves(2), 1486);
    assert_eq!(b.get_num_moves(3), 62379);
    assert_eq!(b.get_num_moves(4), 2103487);
    assert_eq!(b.get_num_moves(5), 89941194);
}

#[test]
fn pos_6_perft() {
    let mut b = Board::from_fen(String::from("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"));
    assert_eq!(b.get_num_moves(1), 46);
    assert_eq!(b.get_num_moves(2), 2079);
    assert_eq!(b.get_num_moves(3), 89890);
    assert_eq!(b.get_num_moves(4), 3894594);
    assert_eq!(b.get_num_moves(5), 164075551);
    assert_eq!(b.get_num_moves(6), 6923051137);
    // too big to test
    //assert_eq!(b.get_num_moves(7),  287188994746 );
    //assert_eq!(b.get_num_moves(8),  11923589843526 );
    //assert_eq!(b.get_num_moves(9),  490154852788714 );
}

#[test]
fn blocks_check() {
    let mut b = Board::from_fen(String::from("1k6/3r4/8/5R2/8/3K4/8/8"));
    assert_eq!(b.get_num_moves(1), 7);
}

#[test]
fn blocks_with_en_passant() {
    let mut b = Board::from_fen(String::from("8/8/8/2k5/4p3/8/3P4/3K4"));
    b.make_move(&Move::new_move(11, 27, false));
    assert_eq!(b.get_num_moves(1), 8);
}

#[test]
fn pinned_rook_can_slide() {
    let mut b = Board::from_fen(String::from("4k3/8/4r3/8/8/4Q3/8/1K6"));
    b.make_move(&Move::new_move(20, 28, false));
    assert_eq!(b.get_num_moves(1), 8);


    let mut b = Board::from_fen(String::from("8/8/2k1r2Q/8/8/8/8/1K6"));
    b.make_move(&Move::new_move(47, 46, false));
    assert_eq!(b.get_num_moves(1), 11);
}

#[test]
fn en_passant_discovered_check() {
    let mut b = Board::from_fen(String::from("8/8/8/8/k2p3Q/8/2P5/2K5"));
    b.make_move(&Move::new_move(10, 26, false));
    assert_eq!(b.get_num_moves(1), 5);
}
