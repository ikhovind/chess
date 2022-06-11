pub mod game;
mod mv;

use std::env;

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
    let args: Vec<String> = env::args().collect();
    let mut board = game::Board::from_fen(
        args[1].to_string()
    );

    let a = board.possible_p( game::FILE_C, 1);
    println!("{}", a.len());
}