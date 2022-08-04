use crate::Board;
use crate::opponent::game_stage::GameStage::EARLY;

pub enum GameStage {
    EARLY = 0,
    MIDDLE = 1,
    LATE = 2
}

pub fn get_game_stage(b: &Board) -> GameStage {
    return EARLY;
}
