#[cfg(test)]
use crate::{Board};
use crate::mv::Move;

#[test]
fn simple_pawn_move() {
    let mut b = Board::from_fen(String::from("k1K/8/8/8/8/8/P7/8"));
    let mut c = Board::from_fen(String::from("k1K/8/8/8/8/P7/8/8"));
    c.white_turn = false;
    let mv = Move::new_move(8, 16, false);
    b.make_move(&mv);
    assert_eq!(b, c);
}
