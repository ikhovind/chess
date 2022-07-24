use rand::Rng;
use crate::Board;
use crate::mv::Move;
extern crate rand;

pub fn eval(b: &Board) -> Option<Move> {
    let moves = b.get_all_moves();
    let mut rng = rand::thread_rng();
    if moves.len() > 0 {
        return Some(moves[rng.gen_range(0..moves.len())]);
    }
    return None;
}