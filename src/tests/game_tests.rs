#[cfg(test)]
use crate::{Board, Move, print_u64_bitboard};
use crate::mv::P_INDEX;
use crate::pieces::{common_moves, king};
use crate::consts::board_consts;
use crate::game::{B_INDEX, K_INDEX, N_INDEX};
use crate::{get_num_moves, pieces};
use crate::pieces::pawn::watched_by_p;

#[test]
fn sliding_moves() {
    let mut b = Board::from_fen(String::from("k7/8/8/8/3R4/8/8/K7"));
    assert_eq!(common_moves::h_and_vmoves(27, b.black_pieces, b.white_pieces),
               board_consts::FILE_MASKS[3] - (1 << 27) + board_consts::RANK_MASKS[3] - (1 << 27));
}

#[test]
fn move_gen() {
    let mut b  = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    assert_eq!(get_num_moves(b,1), 20);
    assert_eq!(get_num_moves(b,2), 400);
    assert_eq!(get_num_moves(b,3), 8902);
    //assert_eq!(get_num_moves(b,4), 197281);
}
