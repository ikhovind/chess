use crate::game::{ANTI_DIAGONAL_MASKS, DIAGONAL_MASKS, FILE_MASKS};
use crate::mv::RANK_MASKS;
use crate::print_u64_bitboard;

pub fn h_and_vmoves(s: u8, opposing_non_k: u64, own_pieces: u64) -> u64 {
    let binary_s:u64 = 1 << s;
    let occupied = (own_pieces | opposing_non_k) - binary_s;
    let possibilities_horizontal: u64 = ((occupied - 2 * binary_s) ^ (occupied.reverse_bits() - 2 * binary_s.reverse_bits()).reverse_bits());
    let possibilities_vertical: u64 = (((occupied & FILE_MASKS[(s as usize) % 8]) - (2 * binary_s)) ^ ((occupied & FILE_MASKS[(s as usize) % 8]).reverse_bits() - (2 * binary_s.reverse_bits())).reverse_bits());
    return (possibilities_horizontal & RANK_MASKS[(s as usize) / 8]) | (possibilities_vertical & FILE_MASKS[(s as usize) % 8]);
}

pub fn d_and_anti_d_moves(s: u8, opposing_non_k: u64, own_pieces: u64) -> u64 {
    let binary_s:u64 = 1 << s;
    let occupied = (own_pieces | opposing_non_k) - binary_s;
    let possibilities_diagonal: u64 = (((occupied) & DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]) - (2 * binary_s)) ^ (((occupied)&DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
    let possibilities_anti_diagonal: u64 = (((occupied)&ANTI_DIAGONAL_MASKS[((s as usize) / 8) + 7 - ((s as usize) % 8)]) - (2 * binary_s)) ^ (((occupied)&ANTI_DIAGONAL_MASKS[((s as usize) / 8) + 7 - ((s as usize) % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
    return (possibilities_diagonal &DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]) | (possibilities_anti_diagonal &ANTI_DIAGONAL_MASKS[((s as usize) / 8) + 7 - ((s as usize) % 8)]);
}

