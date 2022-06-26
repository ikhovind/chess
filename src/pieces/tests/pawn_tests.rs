#[cfg(test)]
mod pawn_tests {
    use crate::{Board, Move, print_u64_bitboard};

    #[test]
    fn simple_en_passant() {
        let mut b = Board::from_fen(String::from("2k5/8/8/8/4p3/8/3P4/2K5"));
        b.make_move(Move::new_double_push(11, 27));
        assert!(
            b.get_all_moves().contains(&Move::new_ep(28, 19))
        );
    }

    #[test]
    fn white_en_passant() {
        let mut b = Board::from_fen(String::from("1k6/4p3/8/8/5P2/8/8/1K6"));
        b.make_move(Move::new_move(29, 37, false));
        b.make_move(Move::new_double_push(52, 36));
        assert!(
            b.get_all_moves().contains(&Move::new_ep(37, 44))
        );
    }
}
