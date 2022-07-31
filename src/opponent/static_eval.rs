use crate::Board;
use crate::consts::board_consts::{B_INDEX, B_VAL_INDEX, K_INDEX, N_INDEX, N_VAL_INDEX, P_INDEX, P_VAL_INDEX, PIECE_VALUES, Q_INDEX, Q_VAL_INDEX, R_INDEX, R_VAL_INDEX};
use crate::opponent::eval_consts::KING_ENDGAME_POS;

pub fn eval_pos(b: &Board) -> i16 {
    let ix = if b.white_turn { 1 } else { 0 };
    return count_pieces(&b.pieces, ix) + weight_king_pos(&b.pieces, ix);
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

    let eval = 14 - (i16::abs(opp_row - our_row) + i16::abs(opp_column - our_column));
    if count_opp_pieces(pieces, ix) < 1000 {
        return KING_ENDGAME_POS[pieces[K_INDEX + 1 - ix].trailing_zeros() as usize] + eval;
    }
    else {
        return KING_ENDGAME_POS[pieces[K_INDEX + 1 - ix].trailing_zeros() as usize] + eval;
    }
}