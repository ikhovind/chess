use std::fs::{File, OpenOptions};
use std::io::{Write};
use std::path::Path;
use std::time::{Instant, SystemTime};
use chrono::{DateTime, Utc};
use num_format::{Locale, WriteFormatted};
use crate::game::Board;
use crate::opponent::engine;

pub mod game;
mod mv;
mod consts;
mod move_gen;
mod opponent;

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


    let mut num = 0;
    for i in 1..(depth) {
        num += b.get_num_moves(i);
    }
    let now = Instant::now();
    {
        num += b.get_num_moves(depth);
    }

    let elapsed = now.elapsed();

    let mut writer = String::new(); // Could also be Vec::new(), File::open(...), ...
    // Write "1,000,000" into the writer...
    writer.write_formatted(&elapsed.as_millis(), &Locale::fr).expect("TODO: panic message");
    println!("num: {}", num);
    let mut writer2 = String::new(); // Could also be Vec::new(), File::open(...), ...
    // Write "1,000,000" into the writer...
    writer2.write_formatted(&((num as u128) / (elapsed.as_millis()) * 1000), &Locale::fr).expect("TODO: panic message");

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
    let res_tmp = format!(" | depth: {}, execution time: {} ms, num per second: {},  date: {}\n",
                          depth,
                          &writer.to_string(),
                          &writer2.to_string(),
                          iso8601(&SystemTime::now()));

    let mut res = fen.clone();
    res.push_str(&*res_tmp);
    file.write_all(res.as_bytes()).expect("ERROR WRITING TO FILE");
}

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%Y-%m-%d"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}

fn main() {
    let mut b = Board::from_fen(
        String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    );

    println!("{}", engine::eval(&b));

    //test(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"), 6);
}