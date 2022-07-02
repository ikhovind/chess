#![allow(unused)]

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;
use num_format::{Locale, WriteFormatted};
use num_format::Locale::el;
use crate::game::Board;
pub mod game;
mod mv;
mod pieces;
mod consts;
mod tests;

fn print_u64_bitboard(bitboard: u64) {
    println!();
    let mut c = 0;
    let mut d = 0;
    let mut str = String::from("");
    for i in format!("{:#066b}", bitboard).chars() {
        if d > 1 {
            str.push(i);
            c += 1;
            if c == 8 {
                print!("{}", str.chars().rev().collect::<String>());
                print!("\n");
                str = String::from("");
                c = 0;
            }
        } else {
            d += 1;
        }
    }
}

fn test(fen: String, depth: u32) {

    let mut b = Board::from_fen(String::from(fen.clone()));
    let now = Instant::now();
    {
        b.get_num_moves(depth);
    }
    let elapsed = now.elapsed();

    let mut writer = String::new(); // Could also be Vec::new(), File::open(...), ...
    // Write "1,000,000" into the writer...
    writer.write_formatted(&elapsed.as_millis(), &Locale::fr);

    let mut file;
    if !Path::new("timestamps.txt").exists() {
        println!("creating");
        file = File::create("timestamps.txt").expect("ERROR READING FROM FILE");
    }
    else {
        file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("timestamps.txt")
            .unwrap();
    };

    //let mut file = File::open("timestamps.txt").expect("ERROR READING FROM FILE");
    let res_tmp = format!(" | depth: {}, execution time: {} ms\n", depth, &writer.to_string());
    let mut res = fen.clone();
    res.push_str(&*res_tmp);
    file.write_all(res.as_bytes()).expect("ERROR WRITING TO FILE");
}

fn main() {
    test(String::from("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - "), 5);
}
