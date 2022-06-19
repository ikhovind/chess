pub mod game;
mod mv;
mod pieces;
mod consts;
mod tests;

use crate::game::Board;
use crate::mv::{Move, R_INDEX};

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
    let b = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));

    println!("{}", get_all_moves(0, &b).len());
}

fn get_all_moves(depth: u32, b: &Board) -> Vec<Move> {
    println!("rook {}", pieces::rook::possible_r(b, b.white_turn).len());
    //println!("{:?}", pieces::rook::possible_r(b, b.white_turn));
    println!("knight {}", pieces::knight::possible_n(b, b.white_turn).len());
    println!("bishop {}", pieces::bishop::possible_b(b, b.white_turn).len());
    println!("queen {}",  pieces::queen::possible_q(b, b.white_turn).len());
    println!("king {}",   pieces::king::possible_k(b, b.white_turn).len());
    println!("pawn {}",   pieces::pawn::possible_p(b, b.white_turn).len());

    let mut rook = pieces::rook::possible_r(b, b.white_turn);
    rook.append(&mut pieces::knight::possible_n(b, b.white_turn));
    rook.append(&mut pieces::bishop::possible_b(b, b.white_turn));
    rook.append(&mut pieces::queen::possible_q(b, b.white_turn));
    rook.append(&mut pieces::king::possible_k(b, b.white_turn));
    rook.append(&mut pieces::pawn::possible_p(b, b.white_turn));
    return rook;
}