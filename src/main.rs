pub mod game;
mod mv;
mod pieces;
mod consts;
mod tests;

use crate::game::Board;
use crate::mv::{ANTI_DIAGONAL_MASKS, DIAGONAL_MASKS, Move, P_INDEX, Q_INDEX, R_INDEX, ROOK};

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
    let mut b  = Board::from_fen(String::from("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  "));
    b.make_move(Move::new_move(1, 18, false));
    b.make_move(Move::new_double_push(48, 32));
    println!("num {}", b.get_all_moves().len());
    //println!("num {}", b.get_num_moves(2));
}
