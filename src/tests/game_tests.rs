#[cfg(test)]
use crate::{Board, Move, print_u64_bitboard};
use crate::mv::P_INDEX;
use crate::pieces::common_moves;
use crate::consts::board_consts;

#[test]
fn sliding_moves() {
    let mut b = Board::from_fen(String::from("k7/8/8/8/3R4/8/8/K7"));
    assert_eq!(common_moves::h_and_vmoves(27, b.black_pieces, b.white_pieces),
               board_consts::FILE_MASKS[3] - (1 << 27) + board_consts::RANK_MASKS[3] - (1 << 27));
}
