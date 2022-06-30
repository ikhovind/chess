#[cfg(test)]
use crate::{Board, Move, pieces, print_u64_bitboard};
use crate::pieces::king;
use crate::pieces::king::possible_k;
use crate::R_INDEX;
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