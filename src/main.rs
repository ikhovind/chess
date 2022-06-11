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
    let mut _pawns = [0,0];
    let mut _bishops = [0,0];
    let mut _rooks = [0,0];
    let mut _knights = [0,0];
    let mut _queens = [0,0];
    let mut _kings = [0,0];

    let args: Vec<String> = env::args().collect();
    let mut column : u32 = 0;
    let mut row = 7;
    let mut res = 0;
    let mut white = 1;
    for i in args[1].chars() {
        if i.is_alphabetic() {
            res = 2_u64.pow((column + row * 8) as u32);
            //print_u64_bitboard(res);
        }
        if !i.is_uppercase() {
            white = 0;
        }
        else {
            white = 1
        }
        match i.to_ascii_lowercase() {
            'p' => {
                column += 1;
                _pawns[white] |= res;
            }
            'r' => {
                column += 1;
                _rooks[white] |= res;
            }
            'n' => {
                column += 1;
                _knights[white] |= res;
            }
            'q' => {
                column += 1;
                _queens[white] |= res;
            }
            'k' => {
                column += 1;
                _kings[white] |= res;
            }
            'b' => {
                column += 1;
                _bishops[white] |= res;
            }
            '/' => {
                row-=1;
                column = 0;
            }
            ' ' => {
                break;
            }
            i if i.is_numeric() => {
                column += i.to_digit(10).unwrap();
            }
            _ => {}
        }
    }
    let mut board = game::Board::new(
        _pawns,
        _knights,
        _bishops,
        _rooks,
        _queens,
        _kings
    );
    board.possible_p( game::FILE_C, 1);
}