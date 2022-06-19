pub use crate::game::board_consts::{RANK_MASKS, FILE_MASKS};
pub use crate::game::board_consts::*;
#[derive(Clone, Copy)]
pub struct Move {
    //smallest 6 bits are to square, bit 7 is promotion, bit 8 is castle, both are ep
    pub from: u8,
    // bit 7 and 8 are type of promotion / type of castle
    pub to: u8
/*
    00 11 - double push pawn

    10 10 - Takes
    11 01 - EP

    01 00 - Queen
    01 01 - Rook
    01 10 - Bishop
    01 11 - Knight

    11 00 - TAKES into Queen
    11 01 - TAKES into Rook
    11 10 - TAKES into Bishop
    11 11 - TAKES into Knight

    10 00 - short castle
    10 01 - long castle
*/
}
pub static BASIS: u8 = 0b00111111;
pub static QUEEN: u8 = 0b00000000;
pub static ROOK: u8 = 0b01000000;
pub static BISHOP: u8 = 0b10000000;
pub static KNIGHT: u8 = 0b11000000;


impl Move {
    pub fn new_move(_from: u8, _to: u8, is_capture: bool) -> Move {
        return Move {from: (_from & BASIS)
                | (if is_capture {TAKES & FROM_MASK} else { 0 }),
            to: (_to & BASIS)
                | (if is_capture {TAKES & TO_MASK} else { 0 })}
    }

    pub fn new_double_push(_from: u8, _to: u8) -> Move {
        return Move {
            from: (_from & BASIS) | (DOUBLE_PAWN & FROM_MASK),
            to: (_to & BASIS) | (DOUBLE_PAWN & TO_MASK)
        }
    }

    pub fn new_promotion(_from: u8, _to: u8, is_capture: bool, promote_to: u8) -> Move {
        let typ;
        if !is_capture {
            // todo error handling
            match (_from & FROM_MASK) & (_from & TO_MASK) {
                0 => typ = PROM_Q,
                1 => typ = PROM_R,
                2 => typ = PROM_B,
                3 => typ = PROM_N,
                _ => typ = PROM_Q
            }
        }
        else {
            // todo error handling
            match (_from & FROM_MASK) & (_from & TO_MASK) {
                0 => typ = TAKE_PROM_Q,
                1 => typ = TAKE_PROM_R,
                2 => typ = TAKE_PROM_B,
                3 => typ = TAKE_PROM_N,
                _ => typ = TAKE_PROM_Q
            }
        }
        return Move {from: (_from & BASIS)
            | (typ & FROM_MASK),
            to: (_to & BASIS)
                | (typ & TO_MASK)
        }
    }

    pub fn new_ep(_from: u8, _to:u8) -> Move{
        return Move {
            from: (_from & BASIS) | (EN_PASSANT & FROM_MASK),
            to: (_to & BASIS) | (EN_PASSANT & TO_MASK)}
    }

    pub fn new_castle(_from: u8, _to:u8) -> Move {
        let typ = if _to == 62 || _to == 6 { SHORT_CASTLE } else { LONG_CASTLE };
        return Move {
            from: (_from & BASIS) | (typ & FROM_MASK),
            to: (_to & BASIS) | (typ & TO_MASK),
        }
    }

    pub fn last_move_was_double_push(m: Move) -> bool {
        return (m.to & TO_MASK) | (m.from & FROM_MASK) == DOUBLE_PAWN ;
    }
}