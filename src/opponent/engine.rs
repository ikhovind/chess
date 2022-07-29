use log::log;
use rand::Rng;
use crate::Board;
use crate::consts::board_consts::{N_INF, P_INF};
use crate::mv::Move;
use crate::opponent::search::{count_material, search_moves};

extern crate rand;

pub fn eval(b: &Board) -> Option<Move> {
    log::info!("thinking about move");
    let moves = b.get_all_moves();
    let mut rng = rand::thread_rng();
    if moves.len() > 0 {
        let mut best_score = i16::MIN;
        let mut best_yet = moves[0];
        for mv in moves {
            //log::info!("Evaluating: {}", mv);
            let curr = -search_moves(b.clone().make_move(&mv), 4, N_INF, P_INF);
            if curr > best_score {
                log::info!("new best move found");
                best_yet = mv;
                best_score = curr;
            }
        }
        log::info!("Returning best move");
        return Some(best_yet);
    }
    return None;
}