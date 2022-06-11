pub use crate::game::board_consts::{FILE_A, FILE_H, RANK_4, RANK_8, FILE_G, FILE_C};
pub struct Move {
    //smallest 6 bits are to square, bit 7 is promotion, bit 8 is castle, both are ep
    from: u8,
    // bit 7 and 8 are type of promotion / type of castle
    to: u8
/*
    10 10 - Takes
    11 01 - EP

    01 00 - Queen
    01 01 - Rook
    01 10 - Bishop
    01 11 - Knight

    11 00 - takes into Queen
    11 01 - takes into Rook
    11 10 - takes into Bishop
    11 11 - takes into Knight

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
                | (if is_capture {0b10000000} else { 0 }),
            to: (_to & BASIS)
                | (if is_capture {0b10000000} else { 0 })}
    }

    pub fn new_promotion(_from: u8, _to: u8, is_capture: bool, promote_to: u8) -> Move {

        return Move {from: (_from & BASIS)
            | (if is_capture {0b11000000} else { 0 })
            | (0b01000000),
            to: (_to & BASIS)
                | (promote_to & 0b11000000)
        }
    }

    pub fn new_ep(_from: u8, _to:u8) -> Move{
        return Move {
            from: (_from & BASIS) | (0b11000000),
            to: (_to & BASIS) | 0b01000000}
    }

    pub fn new_castle(_from: u8, _to:u8) -> Move {
        return Move {
            from: (_from & BASIS) | (0b10000000),
            to: (_to & BASIS) | if _to == 62 || _to == 6 { 0b00000000} else { 0b01000000 }
        }
    }
}