use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, Shutdown, TcpStream};
use std::path::Path;
use std::process::exit;
use std::time::{Instant, SystemTime};
use chrono::{DateTime, Utc};
use num_format::{Locale, WriteFormatted};
use crate::game::Board;
use crate::opponent::engine;
extern crate vampirc_uci;
use simple_websockets::{Event, Responder};
use std::collections::HashMap;

use std::thread;
use vampirc_uci::{MessageList, parse, parse_with_unknown, UciMessage, UciTimeControl};
pub mod game;
mod mv;
mod consts;
mod move_gen;
mod opponent;

//static mut GAME: Board = Board::from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));

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
    // listen for WebSockets on port 8080:
    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");
    // map between client ids and the client's `Responder`:
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                println!("A client connected with id #{}", client_id);
                // add their Responder to our `clients` map:
                clients.insert(client_id, responder);
            },
            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);
                // remove the disconnected client from the clients map:
                clients.remove(&client_id);
            },
            Event::Message(client_id, message) => {
                println!("Received a message from client #{}: {:?}", client_id, message);
                // retrieve this client's `Responder`:
                let responder = clients.get(&client_id).unwrap();
                // echo the message back:
                responder.send(message);
            },
        }
    }
}

fn parser(input: &str) -> Result<&str, &str> {
    let messages: MessageList = parse(&input);

    for m in messages {
        println!("m: {}", m);
        match m {
            UciMessage::Uci => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Position { startpos, fen, moves } => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Go { time_control, search_control } => {
                if let Some(tc) = time_control {
                    match tc {
                        UciTimeControl::Ponder => {
                            return Result::Err("Not implemented yet ");
                        }
                        UciTimeControl::TimeLeft { white_time, white_increment, black_time, black_increment, moves_to_go } => {
                            return Result::Err("Not implemented yet ");
                        }
                        UciTimeControl::Infinite => {
                            return Result::Err("Not implemented yet ");
                        }
                        UciTimeControl::MoveTime(duration) => {
                            return Result::Err("Not implemented yet ");
                        }
                    }
                }
            }
            UciMessage::IsReady => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::UciNewGame => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Stop => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::PonderHit => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Quit => {
                return Result::Err("Not implemented yet ");
            }

            UciMessage::Debug(_) => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Register { .. } => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::SetOption { .. } => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Id { .. } => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::UciOk => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::ReadyOk => {
                return Result::Err("Not implemented yet ");

            }
            UciMessage::BestMove { .. } => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::CopyProtection(_) => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Registration(_) => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Option(_) => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Info(_) => {
                return Result::Err("Not implemented yet ");
            }
            UciMessage::Unknown(_, _) => {
                return Result::Err("Not implemented yet ");
            }
        }
    }
    return Result::Err("No message");
}
