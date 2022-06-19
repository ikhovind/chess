#[cfg(test)]
mod tests {
    use crate::{Board, Move, print_u64_bitboard};
    use crate::mv::P_INDEX;

    #[test]
    fn simple_pawn_move() {
        let mut b = Board::from_fen(String::from("8/8/8/8/8/8/P7/8"));
        let mut c = Board::from_fen(String::from("8/8/8/8/8/P7/8/8"));
        c.white_turn = false;
        let mv = Move::new_move(8, 16, false);
        b.make_move(mv);
        assert_eq!(b, c);
    }
}
