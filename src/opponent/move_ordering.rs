use crate::{Board, Move};
use crate::move_gen::pieces;

pub fn order_moves(b: &Board, moves: &mut Vec<Move>) {
    let mut scores = Vec::new();
    scores.reserve(moves.len());
    for (ix, it) in moves.iter().enumerate() {
        scores.push(-Move::guess_score(it, pieces::pawn::watched_by_p(&b, !b.white_turn), &b));
    }
    let mut permutation = permutation::sort_unstable(&scores);
    permutation.apply_slice_in_place(moves);
}