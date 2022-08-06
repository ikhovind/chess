use crate::{Board, Move};
use crate::move_gen::pieces;

pub fn order_moves(b: &Board, moves: &mut Vec<Move>) {
    let mut scores = Vec::new();
    scores.reserve(moves.len());
    let watched_by_p = pieces::pawn::watched_by_p(b, !b.white_turn);
    for it in moves.iter() {
        scores.push(-Move::guess_score(it, watched_by_p, b));
    }
    let mut permutation = permutation::sort_unstable(&scores);
    permutation.apply_slice_in_place(moves);
}