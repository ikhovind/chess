use crate::{Board, Move};
use crate::opponent::engine::eval;
use crate::opponent::game::Game;
use crate::opponent::game_stage::GameStage::LATE;

#[test]
fn finds_mate_in_one() {
    // https://www.chessprogramming.org/Perft_Results
    let mut b = Game::from_fen("2k5/8/2K5/r7/8/8/8/4r3 b - - 0 1");
    b.board.white_turn = false;
    b.stage = LATE;
    assert_eq!(eval(&mut b, 4).unwrap(), Move::new_move(4, 44, false));
}
