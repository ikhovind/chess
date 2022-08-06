extern crate core;
extern crate log;

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use warp::Filter;

use crate::board::Board;
use crate::mv::Move;

pub mod board;
pub mod mv;
pub mod consts;
pub mod move_gen;
pub mod opponent;



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

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}