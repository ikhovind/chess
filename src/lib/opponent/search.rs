use std::clone;
use crate::Board;
use crate::consts::board_consts::N_MATE;
use crate::move_gen::pieces;
use crate::opponent::game_stage::GameStage;
use crate::opponent::move_ordering::order_moves;
use crate::opponent::static_eval::eval_pos;

const PAWN_VALUE: u32 = 100;
const QUEEN_VALUE: u32 = 900;
const KNIGHT_VALUE: u32 = 300;
const BISHOP_VALUE: u32 = 300;
const ROOK_VALUE: u32 = 500;


pub fn search_moves(mut b: Board, depth: u8, mut alpha: i16, beta: i16, stage: GameStage) -> i16 {
    if depth == 0 {
        return quiescence_search(b, alpha, beta, stage);
    }
    let mut moves = b.get_all_moves();
    if moves.len() == 0 {
        if pieces::king::get_attackers(&b, b.white_turn) != 0 {
            return N_MATE - depth as i16;
        }
        return 0;
    }
    else {
        order_moves(&b, &mut moves);
        for mv in moves {
            let evaluation = -search_moves(b.make_move(&mv), depth - 1, -beta, -alpha, stage);
            // opponent has a better choice, can prune
            if evaluation >= beta {
                return beta;
            }
            if evaluation > alpha {
                alpha = evaluation;
            }
        }
        return alpha;
    }
}


fn quiescence_search(b: Board, mut alpha: i16, beta: i16, stage: GameStage) -> i16 {
    let mut eval = eval_pos(&b, &stage);
    if eval >= beta {
        return beta;
    }
    if eval > alpha {
        alpha = eval;
    }

    let mut moves = b.get_all_captures();
    order_moves(&b, &mut moves);
    for mv in moves {
        eval = -quiescence_search(b.make_move(&mv), -beta, -alpha, stage);
        // opponent has a better choice, can prune
        if eval >= beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }
    }
    return alpha;
}