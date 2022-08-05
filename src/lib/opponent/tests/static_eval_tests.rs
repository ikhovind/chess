use crate::Board;
use crate::consts::board_consts::N_INDEX;
use crate::opponent::eval_consts::eval_sq;
use crate::opponent::game_stage::GameStage;
use crate::opponent::game_stage::GameStage::EARLY;
use crate::opponent::static_eval::eval_pos;

#[test]
fn eval_single_knight() {
    let eval = eval_sq(49, N_INDEX, true);
    let eval_white = eval_sq(9, N_INDEX + 1, true);
    assert_eq!(eval, eval_white);
}

