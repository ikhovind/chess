use crate::{Board, Move};
use crate::opponent::engine::eval;

#[test]
fn finds_mate_in_one() {
    // https://www.chessprogramming.org/Perft_Results
    let mut b = Board::from_fen(String::from("2k5/8/2K5/r7/8/8/8/4r3 b - - 0 1"));
    b.white_turn = false;
    assert_eq!(eval(b, 4).unwrap(), Move::new_move(4, 44, false));
}
