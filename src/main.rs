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
    let mut b  = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    println!("sum: {}", get_num_moves(b,2));

}

fn get_all_moves(b: &Board) -> Vec<Move> {

    /*
    println!("rook: {}", pieces::rook::possible_r(b, b.white_turn).len());
    println!("knight: {}", pieces::knight::possible_n(b, b.white_turn).len());
    println!("bishop: {}", pieces::bishop::possible_b(b, b.white_turn).len());
    println!("queen: {}", pieces::queen::possible_q(b, b.white_turn).len());
    println!("king: {}", pieces::king::possible_k(b, b.white_turn).len());
    println!("pawn: {}", pieces::pawn::possible_p(b, b.white_turn).len());

     */
    let mut rook = pieces::rook::possible_r(b, b.white_turn);
    rook.append(&mut pieces::knight::possible_n(b, b.white_turn));
    rook.append(&mut pieces::bishop::possible_b(b, b.white_turn));
    rook.append(&mut pieces::queen::possible_q(b, b.white_turn));
    rook.append(&mut pieces::king::possible_k(b, b.white_turn));
    rook.append(&mut pieces::pawn::possible_p(b, b.white_turn));
    return rook;
}

fn get_num_moves(b: Board, depth: u32) -> u64 {
    let mut sum = 0;
    let b2 = b.clone();
    if depth == 0 {
        return get_all_moves(&b2).len() as u64;
    }
    for nw in get_all_moves(&b) {
        let &mut test = b.clone().make_move(nw);
        let &mut test2 = b.clone().make_move(nw);
        sum += get_num_moves(test, depth - 1);
        if depth == 1 { println!("{}: {}", nw.to_string(), get_num_moves(test2, depth - 1));}
    }
    return sum;
}