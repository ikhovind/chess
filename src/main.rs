pub mod game;
mod mv;
mod pieces;
mod consts;
mod tests;

use crate::game::Board;
use crate::mv::{Move, P_INDEX, Q_INDEX, R_INDEX};

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
    let mut b  = Board::from_fen(String::from("rnbqkbnr/ppp1pppp/8/3p4/8/2P5/PP1PPPPP/RNBQKBNR"));
    b.make_move(Move::new_move(3, 24, false));
    println!("{}", b.get_num_moves(1));

}
