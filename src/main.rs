pub mod game;
mod mv;
mod pieces;

use std::env;
use crate::game::Board;
use crate::mv::Move;

fn print_u64_bitboard(bitboard : u64) {
    println!();
    let mut c = 0;
    let mut d = 0;
    let mut str = String::from("");
    for i in format!("{:#066b}", bitboard).chars() {
        if d > 1 {
            str.push(i);
            c +=1;
            if c == 8 {
                print!("{}", str.chars().rev().collect::<String>());
                print!("\n");
                str = String::from("");
                c = 0;
            }
        }
        else {
            d += 1;
        }
    }
}

fn main() {
    let b = Board::from_fen(String::from("2p5/3K4/8/4n3/8/8/8/8"));
    print_u64_bitboard(pieces::king::get_attackers(&b, true));
}