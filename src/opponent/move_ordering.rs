use crate::{Board, Move};
use crate::move_gen::pieces;

pub fn order_moves(b: &Board, moves: &Vec<Move>) -> Vec<Move> {
    let mut scores = Vec::new();
    scores.reserve(moves.len());
    for (ix, it) in moves.iter().enumerate() {
        scores.push(Move::guess_score(it, pieces::pawn::watched_by_p(&b, b.white_turn), &b));
    }
    let permutation = permutation::sort_unstable(&scores);
    return permutation.apply_slice(&moves);
}