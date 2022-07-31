use log::log;
use num_format::Locale::el;
use crate::{Board, print_u64_bitboard};
use crate::consts::board_consts::{N_INF, P_INF};
use crate::mv::Move;
use crate::opponent::move_ordering::order_moves;
use crate::opponent::search::search_moves;
use crate::opponent::static_eval::{eval_pos, weight_king_pos};

pub fn eval(b: &Board) -> Option<Move> {
    log::info!("thinking about move");
    let mut moves = b.get_all_moves();
    order_moves(&b, &mut moves);
    if moves.len() > 0 {
        println!("move: {}", weight_king_pos(&b.pieces, if b.white_turn { 1 } else { 0 }));
        let mut best_score = i16::MIN;
        let mut best_yet = moves[0];
        for mv in moves {
            log::info!("Evaluating: {}", mv);
            let curr = -search_moves(&b.clone().make_move(&mv), 4, N_INF, P_INF);
            if curr > best_score {
                log::info!("new best move found");
                best_yet = mv;
                best_score = curr;
            }
        }
        log::info!("Returning best move with score: {}", best_score);
        return Some(best_yet);
    }
    return None;
}