use crate::consts::board_consts::{ANTI_DIAGONAL_MASKS, DIAGONAL_MASKS, FILE_MASKS, RANK_MASKS};
use crate::Move;

pub fn h_and_vmoves(s: u8, opposing_non_k: u64, own_pieces: u64) -> u64 {
    let binary_s: u64 = 1 << s;
    let mut occupied = own_pieces | opposing_non_k;
    if occupied & binary_s != 0 { occupied -= binary_s; }
    let possibilities_horizontal: u64 = (occupied - 2 * binary_s) ^ (occupied.reverse_bits() - 2 * binary_s.reverse_bits()).reverse_bits();
    let possibilities_vertical: u64 = ((occupied & FILE_MASKS[(s as usize) % 8]) - (2 * binary_s)) ^ ((occupied & FILE_MASKS[(s as usize) % 8]).reverse_bits() - (2 * binary_s.reverse_bits())).reverse_bits();

    (possibilities_horizontal & RANK_MASKS[(s as usize) / 8]) | (possibilities_vertical & FILE_MASKS[(s as usize) % 8])
}

pub fn d_and_anti_d_moves(s: u8, opposing_non_k: u64, own_pieces: u64) -> u64 {
    let binary_s: u64 = 1 << s;
    let mut occupied = own_pieces | opposing_non_k;
    if occupied & binary_s != 0 { occupied -= binary_s; }
    let possibilities_diagonal: u64 = (((occupied) & DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]) - (2 * binary_s)) ^ (((occupied) & DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
    let possibilities_anti_diagonal: u64 = (((occupied) & ANTI_DIAGONAL_MASKS[((s / 8) + (7 - s % 8)) as usize]) - (2 * binary_s)) ^ (((occupied) & ANTI_DIAGONAL_MASKS[((s as usize) / 8) + 7 - ((s as usize) % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
    (possibilities_diagonal & DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]) | (possibilities_anti_diagonal & ANTI_DIAGONAL_MASKS[((s as usize) / 8) + (7 - ((s as usize) % 8))])
}

pub fn add_moves_to_list(opp: u64, list: &mut Vec<Move>, i: u32, pieces: u64) {
    for i2 in (pieces.trailing_zeros())..(64 - pieces.leading_zeros()) {
        if (1 << i2) & pieces != 0 {
            list.push(
                Move::new_move(i as u8, i2 as u8, opp & (1 << i2) != 0)
            );
        }
    }
}
