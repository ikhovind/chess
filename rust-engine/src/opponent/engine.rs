use rand::Rng;
use crate::Board;
use crate::mv::Move;
extern crate rand;

pub fn eval(b: &Board) -> Move {
    let moves = b.get_all_moves();
    let mut rng = rand::thread_rng();
    return moves[rng.gen_range(0..moves.len())];
}