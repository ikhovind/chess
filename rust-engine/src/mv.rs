use std::fmt;

use crate::consts::board_consts::*;

#[derive(Clone, Copy, Debug)]
pub struct Move {
    //smallest 6 bits are to square, bit 7 is promotion, bit 8 is castle, both are ep
    pub from: u8,
    // bit 7 and 8 are type of promotion / type of castle
    pub to: u8,
    /*
    pub const NORMAL_MOVE: u8 = 0;
    pub const DOUBLE_PAWN: u8 = 0b00010000;
    pub const TAKES: u8       = 0b00100000;
    pub const EN_PASSANT: u8 =  0b00110000; // - EP

    pub const PROM_Q: u8      = 0b01000000; // - Queen
    pub const PROM_R: u8      = 0b01010000; // - Rook
    pub const PROM_B: u8      = 0b01100000; // - Bishop
    pub const PROM_N: u8      = 0b01110000; // - Knight

    pub const TAKE_PROM_Q: u8 = 0b10000000; // - TAKES into Queen
    pub const TAKE_PROM_R: u8 = 0b10010000; // - TAKES into Rook
    pub const TAKE_PROM_B: u8 = 0b10100000; // - TAKES into Bishop
    pub const TAKE_PROM_N: u8 = 0b10110000; // - TAKES into Knight

    pub const SHORT_CASTLE: u8= 0b11000000; // - short castle
    pub const LONG_CASTLE: u8 = 0b11010000; // - long castle
    */
}

pub static BASIS: u8 = 0b00111111;


impl Move {
    pub fn new_move(_from: u8, _to: u8, is_capture: bool) -> Move {
        return Move {
            from: (_from & BASIS)
                | (if is_capture { TAKES & FROM_MASK } else { 0 }),
            to: (_to & BASIS)
                | (if is_capture { TAKES << 2 } else { 0 }),
        };
    }

    pub fn new_double_push(_from: u8, _to: u8) -> Move {
        let mv = Move {
            from: (_from & BASIS) | (DOUBLE_PAWN & FROM_MASK),
            to: (_to & BASIS) | ((DOUBLE_PAWN & TO_MASK) << 2),
        };
        return mv;
    }

    pub fn new_promotion(_from: u8, _to: u8, is_capture: bool, promote_to: u8) -> Move {
        let typ;
        if !is_capture {
            // todo error handling
            match promote_to {
                0 => typ = PROM_Q,
                1 => typ = PROM_R,
                2 => typ = PROM_B,
                3 => typ = PROM_N,
                _ => typ = PROM_Q
            }
        } else {
            // todo error handling
            match promote_to {
                0 => typ = TAKE_PROM_Q,
                1 => typ = TAKE_PROM_R,
                2 => typ = TAKE_PROM_B,
                3 => typ = TAKE_PROM_N,
                _ => typ = TAKE_PROM_Q
            }
        }
        return Move {
            from: (_from & BASIS) | (typ & FROM_MASK),
            to: (_to & BASIS) | (typ << 2),
        };
    }

    pub fn new_ep(_from: u8, _to: u8) -> Move {
        let m = Move {
            from: (_from & BASIS) | (EN_PASSANT & FROM_MASK),
            to: (_to & BASIS) | (EN_PASSANT << 2),
        };
        return m;
    }

    pub fn new_castle(_from: u8, _to: u8) -> Move {
        let typ = if _to == 62 || _to == 6 { SHORT_CASTLE } else { LONG_CASTLE };
        return Move {
            from: (_from & BASIS) | (typ & FROM_MASK),
            to: (_to & BASIS) | (typ << 2),
        };
    }

    pub fn last_move_was_double_push(m: Move) -> bool {
        return (((m.to & !MOVE_MASK) >> 2) | (m.from & !MOVE_MASK)) == DOUBLE_PAWN;
    }
}

impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match (self.from & MOVE_MASK) % 8 {
            0 => { fmt.write_str("a")?; }
            1 => { fmt.write_str("b")?; }
            2 => { fmt.write_str("c")?; }
            3 => { fmt.write_str("d")?; }
            4 => { fmt.write_str("e")?; }
            5 => { fmt.write_str("f")?; }
            6 => { fmt.write_str("g")?; }
            7 => { fmt.write_str("h")?; }
            _ => { fmt.write_str("error")?; }
        }
        match (self.from & MOVE_MASK) / 8 {
            0 => { fmt.write_str("1")?; }
            1 => { fmt.write_str("2")?; }
            2 => { fmt.write_str("3")?; }
            3 => { fmt.write_str("4")?; }
            4 => { fmt.write_str("5")?; }
            5 => { fmt.write_str("6")?; }
            6 => { fmt.write_str("7")?; }
            7 => { fmt.write_str("8")?; }
            _ => { fmt.write_str("error")?; }
        }
        match (self.to & MOVE_MASK) % 8 {
            0 => { fmt.write_str("a")?; }
            1 => { fmt.write_str("b")?; }
            2 => { fmt.write_str("c")?; }
            3 => { fmt.write_str("d")?; }
            4 => { fmt.write_str("e")?; }
            5 => { fmt.write_str("f")?; }
            6 => { fmt.write_str("g")?; }
            7 => { fmt.write_str("h")?; }
            _ => { fmt.write_str("error")?; }
        }
        match (self.to & MOVE_MASK) / 8 {
            0 => { fmt.write_str("1")?; }
            1 => { fmt.write_str("2")?; }
            2 => { fmt.write_str("3")?; }
            3 => { fmt.write_str("4")?; }
            4 => { fmt.write_str("5")?; }
            5 => { fmt.write_str("6")?; }
            6 => { fmt.write_str("7")?; }
            7 => { fmt.write_str("8")?; }
            _ => { fmt.write_str("error")?; }
        }
        match self.from & TYPE_MASK | ((self.to & TYPE_MASK) >> 2) {
            PROM_Q => {
                fmt.write_str("q").expect("ERR");
            }
            PROM_R => { fmt.write_str("r").expect("ERR"); }
            PROM_B => { fmt.write_str("b").expect("ERR"); }
            PROM_N => { fmt.write_str("n").expect("ERR"); }
            TAKE_PROM_Q => {
                fmt.write_str("q").expect("ERR");
            }
            TAKE_PROM_R => { fmt.write_str("r").expect("ERR"); }
            TAKE_PROM_B => { fmt.write_str("b").expect("ERR"); }
            TAKE_PROM_N => { fmt.write_str("n").expect("ERR"); }
            _ => {}
        }
        Ok(())
    }
}


impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
            && self.to == other.to
    }
}