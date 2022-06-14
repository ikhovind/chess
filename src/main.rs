pub mod game;
mod mv;
mod pieces;

use std::env;
use crate::game::Board;
use crate::mv::Move;
use crate::pieces::king;
use crate::pieces::pawn::{attacked_from_square, pawn2, watched_by_p};

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
    let board = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    print_u64_bitboard(pawn2::attacked_from_square(48, false));
}