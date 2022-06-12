pub mod game;
mod mv;

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
    let board = Board::from_fen(String::from("rnb1kbnr/pppppppp/8/8/7q/4PP2/PPP1PKPP/RNBQ1BNR"));
    print_u64_bitboard(board.watched(false));
    let a = board.possible_k( true);
    println!("{}", a.len());
}