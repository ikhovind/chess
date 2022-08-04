use crate::Board;
use crate::consts::board_consts::{B_INDEX, B_VAL_INDEX, K_INDEX, N_INDEX, N_VAL_INDEX, P_INDEX, P_VAL_INDEX, PIECE_VALUES, Q_INDEX, Q_VAL_INDEX, R_INDEX, R_VAL_INDEX};
use crate::opponent::eval_consts::{EG_KING_TABLE, eval_sq};
use crate::opponent::game_stage::GameStage;

pub fn eval_pos(b: &Board, stage: &GameStage) -> i16 {
    let ix = if b.white_turn { 1 } else { 0 };
    // todo perspektiv her ikke inne i funksjonene
    return match stage {
        GameStage::EARLY => {
            count_pieces(&b.pieces, ix) + eval_piece_positions(&b.pieces, b.white_turn, true)
        }
        GameStage::MIDDLE => {
            count_pieces(&b.pieces, ix) + eval_piece_positions(&b.pieces, b.white_turn, false)
        }
        GameStage::LATE => {
            count_pieces(&b.pieces, ix) + weight_king_pos(&b.pieces, ix)
        }
    }
}

fn eval_piece_positions(pieces: &[u64; 12], white_turn: bool, early_game: bool) -> i16 {
    let mut white = 0;
    let mut black = 0;
    for piece in 0..12 {
        for square in (pieces[piece].trailing_zeros() as usize)..(64usize - pieces[piece].leading_zeros() as usize) {
            if pieces[piece] & (1 << square) != 0 {
                if piece % 2 == 0 {
                    black -= eval_sq(square, piece / 2, early_game);
                }
                else {
                    white += eval_sq(square, piece / 2, early_game);
                }
            }
        }
    }
    let perspective = if white_turn { 1  } else { -1 };
    return (white - black) * perspective;
}

fn count_pieces(pieces: &[u64; 12], ix: usize) -> i16 {
    return (pieces[P_INDEX + ix].count_ones() as i16 * PIECE_VALUES[P_VAL_INDEX]
        - pieces[P_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[P_VAL_INDEX]  +
        pieces[Q_INDEX + ix].count_ones() as i16 * PIECE_VALUES[Q_VAL_INDEX]
        - pieces[Q_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[Q_VAL_INDEX]  +
        pieces[N_INDEX + ix].count_ones() as i16 * PIECE_VALUES[N_VAL_INDEX]
        - pieces[N_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[N_VAL_INDEX]  +
        pieces[B_INDEX + ix].count_ones() as i16 * PIECE_VALUES[B_VAL_INDEX]
        - pieces[B_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[B_VAL_INDEX]  +
        pieces[R_INDEX + ix].count_ones() as i16 * PIECE_VALUES[R_VAL_INDEX]
        - pieces[R_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[R_VAL_INDEX]) as i16;
}

fn count_opp_pieces(pieces: &[u64; 12], ix: usize) -> i16 {
    return (pieces[P_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[P_VAL_INDEX]
        + pieces[Q_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[Q_VAL_INDEX]
        + pieces[N_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[N_VAL_INDEX]
        + pieces[B_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[B_VAL_INDEX]
        + pieces[R_INDEX + 1 - ix].count_ones() as i16 * PIECE_VALUES[R_VAL_INDEX]) as i16;
}

pub fn weight_king_pos(pieces: &[u64; 12], ix: usize) -> i16 {
    let our_king_sq = pieces[K_INDEX + ix].trailing_zeros() as i16;
    let opp_king_sq = pieces[K_INDEX + 1 - ix].trailing_zeros() as i16;
    let our_row = our_king_sq / 8;
    let our_column = our_king_sq % 8;

    let opp_row = opp_king_sq / 8;
    let opp_column = opp_king_sq % 8;

    let dist =  (i16::abs(opp_row - our_row) + i16::abs(opp_column - our_column));
    // todo tweak endgame;
    if count_opp_pieces(pieces, ix) < 1000 {
        return EG_KING_TABLE[pieces[K_INDEX + ix].trailing_zeros() as usize] + dist;
    }
    else {
        return -dist;
    }
}