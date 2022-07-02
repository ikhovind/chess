pub mod game;
mod mv;
mod pieces;
mod consts;
mod tests;

use crate::game::Board;
use crate::mv::{ANTI_DIAGONAL_MASKS, DIAGONAL_MASKS, KNIGHT, Move, N_INDEX, P_INDEX, Q_INDEX, QUEEN, R_INDEX, ROOK};

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
    let mut b = Board::from_fen(String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - "));
    /*
    b.make_move(Move::new_move(25, 1, false));
    b.make_move(Move::new_move(31, 38, false));
    b.make_move(Move::new_move(1, 6, false));
    b.make_move(Move::new_double_push(50, 34));
    b.make_move(Move::new_move(8, 16, false));
    b.make_move(Move::new_move(40, 33, false));
    b.make_move(Move::new_move(36, 51, true));
    b.make_move(Move::new_move(42, 57, false));
    b.make_move(Move::new_double_push(9, 25));
    b.make_move(Move::new_move(25, 33, false));
    b.make_move(Move::new_double_push(48, 32));
    b.make_move(Move::new_double_push(51, 35));
    b.make_move(Move::new_move(3, 24, false));

     */
    println!("num {}", b.get_num_moves(6));
    //println!("num {}", b.get_all_moves().len());
}
