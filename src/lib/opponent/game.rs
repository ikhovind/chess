use crate::Board;
use crate::consts::board_consts::{B_INDEX, B_VAL_INDEX, N_INDEX, N_VAL_INDEX, P_INDEX, P_VAL_INDEX, PIECE_VALUES, Q_INDEX, Q_VAL_INDEX, R_INDEX, R_VAL_INDEX};
use crate::opponent::game_stage::GameStage;
use crate::opponent::game_stage::GameStage::{EARLY, LATE, MIDDLE};

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub history: String,
    pub stage: GameStage,
}

impl Game {
    pub fn from_fen(fen: &str) -> Game {
        let b = Board::from_fen(String::from(fen));
        return Game {
            board: b,
            history: String::new(),
            stage: EARLY,
        }
    }
    pub fn set_stage(&mut self) {
        match self.stage {
            EARLY => {
            }
            MIDDLE => {
                if self.count_pieces() < 10 {
                    self.stage = LATE;
                }
            }
            LATE => {}
        }
    }

    fn count_pieces(&self) -> u8 {
        return (self.board.pieces[P_INDEX].count_ones() +
            self.board.pieces[P_INDEX + 1].count_ones() +
            self.board.pieces[Q_INDEX].count_ones()     +
            self.board.pieces[Q_INDEX + 1].count_ones() +
            self.board.pieces[N_INDEX].count_ones()     +
            self.board.pieces[N_INDEX + 1].count_ones() +
            self.board.pieces[B_INDEX].count_ones()     +
            self.board.pieces[B_INDEX + 1].count_ones() +
            self.board.pieces[R_INDEX].count_ones()     +
            self.board.pieces[R_INDEX + 1].count_ones()) as u8
    }
}
