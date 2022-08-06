use crate::Board;
use crate::opponent::game_stage::GameStage::EARLY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStage {
    EARLY = 0,
    MIDDLE = 1,
    LATE = 2
}

