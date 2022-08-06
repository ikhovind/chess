#[cfg(test)]
use crate::{Board};
use crate::Move;


#[test]
fn simple_pawn_move() {
    let mut b = Board::from_fen(String::from("k1K/8/8/8/8/8/P7/8"));
    let mut c = Board::from_fen(String::from("k1K/8/8/8/8/P7/8/8"));
    c.white_turn = false;
    let mv = Move::new_move(8, 16, false);
    assert_eq!(b.make_move(&mv), c);
}
