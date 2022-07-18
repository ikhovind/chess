use std::cmp::{max, min};
use crate::consts::board_consts::{ANTI_DIAGONAL_MASKS, DIAGONAL_MASKS, FILE_MASKS, RANK_MASKS};
use crate::print_u64_bitboard;

pub fn h_and_vmoves(s: u8, opposing_non_k: u64, own_pieces: u64) -> u64 {
    let binary_s: u64 = 1 << s;
    let mut occupied = (own_pieces | opposing_non_k);
    if occupied & binary_s != 0 { occupied -= binary_s; }
    let possibilities_horizontal: u64 = ((occupied - 2 * binary_s) ^ (occupied.reverse_bits() - 2 * binary_s.reverse_bits()).reverse_bits());
    let possibilities_vertical: u64 = (((occupied & FILE_MASKS[(s as usize) % 8]) - (2 * binary_s)) ^ ((occupied & FILE_MASKS[(s as usize) % 8]).reverse_bits() - (2 * binary_s.reverse_bits())).reverse_bits());

    return (possibilities_horizontal & RANK_MASKS[(s as usize) / 8]) | (possibilities_vertical & FILE_MASKS[(s as usize) % 8]);
}

pub fn d_and_anti_d_moves(s: u8, opposing_non_k: u64, own_pieces: u64) -> u64 {
    let binary_s: u64 = 1 << s;
    let mut occupied = own_pieces | opposing_non_k;
    if occupied & binary_s != 0 { occupied -= binary_s; }
    let possibilities_diagonal: u64 = (((occupied) & DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]) - (2 * binary_s)) ^ (((occupied) & DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
    let possibilities_anti_diagonal: u64 = (((occupied) & ANTI_DIAGONAL_MASKS[((s / 8) + (7 - s % 8)) as usize]) - (2 * binary_s)) ^ (((occupied) & ANTI_DIAGONAL_MASKS[((s as usize) / 8) + 7 - ((s as usize) % 8)]).reverse_bits() - (2 * (binary_s).reverse_bits())).reverse_bits();
    return (possibilities_diagonal & DIAGONAL_MASKS[((s as usize) / 8) + ((s as usize) % 8)]) | (possibilities_anti_diagonal & ANTI_DIAGONAL_MASKS[((s as usize) / 8) + (7 - ((s as usize) % 8))]);
}


pub fn anti_diag_ray(from_square: u8, to_square: u8) -> u64 {
    let binary_s: u64 = 1 << to_square;
    let occ = 1 << from_square;
    return ANTI_DIAGONAL_MASKS[((to_square as usize) / 8) + 7 - ((to_square as usize) % 8)] & (occ ^ (occ - 2 * binary_s));
}

pub fn diag_ray(from_square: u8, to_square: u8) -> u64 {
    let binary_s: u64 = 1 << to_square;
    let occ = 1 << from_square;
    return DIAGONAL_MASKS[((to_square as usize) / 8) + ((to_square as usize) % 8)] & (occ ^ (occ - 2 * binary_s));
}

pub fn h_moves(from_sq: u8, to_sq: u8) -> u64 {
    let binary_s: u64 = 1 << to_sq;
    let occ = 1 << from_sq;
    return  RANK_MASKS[(to_sq as usize) / 8] & (occ ^ (occ - 2 * binary_s));
}

pub fn v_moves(from_sq: u8, to_sq: u8) -> u64 {
    let binary_s: u64 = 1 << to_sq;
    let occ = 1 << from_sq;
    return  FILE_MASKS[(to_sq as usize) % 8] & (occ ^ (occ - 2 * binary_s));
}

pub fn ray_between(attacker: u8, piece_square: u8) -> u64 {
    // same column
    // column
    return if attacker % 8 == piece_square % 8 {
        ((1 << attacker) & !(1 << piece_square)) | (v_moves(max(attacker, piece_square), min(attacker, piece_square)))
    }
    // same row
    else if attacker / 8 == piece_square / 8 {
        ((1 << attacker) & !(1 << piece_square)) | h_moves(max(attacker, piece_square), min(attacker, piece_square))
    }
    // diagonal
    else {
        // to the left
        if u8::abs_diff(attacker, piece_square) % 7 == 0 {
            let max = max(attacker, piece_square);
            let min = min(attacker, piece_square);
            diag_ray(max, min) | (((1 << attacker)) & !(1 << piece_square))
        }
        // to the right
        else {
            let max = max(attacker, piece_square);
            let min = min(attacker, piece_square);
            diag_ray(max, min) | (((1 << attacker)) & !(1 << piece_square))
        }
    }
}

