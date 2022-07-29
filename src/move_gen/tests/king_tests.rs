#[cfg(test)]
use crate::{Board};
use crate::consts::board_consts::K_INDEX;
#[cfg(test)]
use crate::move_gen::pieces;
use crate::move_gen::pieces::king;
use crate::mv::Move;
use crate::move_gen::pieces::king::possible_k;
// Note this useful idiom: importing names from outer (for mod tests) scope.

#[test]
fn detects_actual_double_check() {
    let b = Board::from_fen(String::from("2p5/3K4/8/4n3/8/8/8/k7"));
    assert_eq!(pieces::king::is_double_check(pieces::king::get_attackers(&b, true)), true);
}


#[test]
fn does_not_detect_single_as_double_check() {
    let b = Board::from_fen(String::from("8/3K4/8/4n3/8/8/8/8"));
    assert_eq!(pieces::king::is_double_check(pieces::king::get_attackers(&b, true)), false);
}

#[test]
fn simple_king_check() {
    let b = Board::from_fen(String::from("k7/8/8/8/8/8/8/5rK1 w - - 0 1"));
    let push_mask = b.ray_between(king::get_attackers(&b, b.white_turn).trailing_zeros() as u8, b.pieces[(K_INDEX + 1) as usize].trailing_zeros() as u8);
    assert_eq!(push_mask, 1u64 << 5);
}


#[test]
fn does_not_detect_none_as_double_check() {
    let b = Board::from_fen(String::from("8/3K4/8/8/8/8/8/8"));
    assert_eq!(pieces::king::is_double_check(pieces::king::get_attackers(&b, true)), false);
}

#[test]
fn evades_simple_check() {
    let mut b = Board::from_fen(String::from("N1R5/3k4/8/2R1R3/8/8/8/1K6"));
    b.white_turn = false;
    assert_eq!(possible_k(&b, false).len(), 1);
    assert_eq!(possible_k(&b, false)[0], Move::new_move(51, 43, false));
}