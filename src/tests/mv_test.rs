#[cfg(test)]
mod tests {
    use crate::{Board, Move, print_u64_bitboard};

    #[test]
    fn simple_pawn_move() {
        let mut b = Board::from_fen(String::from("8/8/8/8/8/8/P7/8"));
        let c = Board::from_fen(String::from("8/8/8/8/8/P7/8/8"));
        let mv = Move::new_move(8, 16, false);
        print_u64_bitboard(b.white_pieces);
        b.make_move(mv);
        print_u64_bitboard(b.pieces[1]);
    }
}
