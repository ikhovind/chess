#[cfg(test)]
mod tests {
    use crate::{Board, pieces, print_u64_bitboard};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn detects_actual_double_check() {
        let b = Board::from_fen(String::from("2p5/3K4/8/4n3/8/8/8/8"));
        print_u64_bitboard(pieces::king::get_attackers(&b, true));
        assert_eq!(pieces::king::is_double_check(pieces::king::get_attackers(&b, true)), true);
    }


    #[test]
    fn does_not_detect_single_as_double_check() {
        let b = Board::from_fen(String::from("8/3K4/8/4n3/8/8/8/8"));
        print_u64_bitboard(pieces::king::get_attackers(&b, true));
        assert_eq!(pieces::king::is_double_check(pieces::king::get_attackers(&b, true)), false);
    }


    #[test]
    fn does_not_detect_none_as_double_check() {
        let b = Board::from_fen(String::from("8/3K4/8/8/8/8/8/8"));
        print_u64_bitboard(pieces::king::get_attackers(&b, true));
        assert_eq!(pieces::king::is_double_check(pieces::king::get_attackers(&b, true)), false);
    }
}