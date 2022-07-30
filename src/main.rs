use std::fs::{File, OpenOptions};
use std::io::{Write};
use std::path::Path;
use std::time::{Instant, SystemTime};
use chrono::{DateTime, Utc};
use num_format::{Locale, WriteFormatted};
use crate::game::Board;
use crate::opponent::engine;
extern crate core;
extern crate log;

use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use std::{env, future};
use std::net::TcpStream;
use std::thread::Builder;
use log::{error, info, LevelFilter, log};
use log::LevelFilter::Info;
use num_format::Locale::{es, my};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use warp::sse::keep_alive;
use crate::consts::board_consts::{ANTI_DIAGONAL_MASKS, BASE_POS, DIAGONAL_MASKS};
use crate::engine::eval;
use crate::mv::Move;

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


/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;
type Games = HashMap<usize, Board>;

#[tokio::main]
async fn main() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
        // - and per-module overrides
        .level_for("rustls", log::LevelFilter::Warn)
        .level_for("hyper", log::LevelFilter::Warn)
        .level_for("tungstenite", log::LevelFilter::Warn)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply().unwrap();    // Keep track of all connected users, key is usize, value

    // is a websocket sender.
    let users = Users::default();
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());

    let games = Games::default();

    // Turn our "state" into a new Filter...
    let games = warp::any().map(move || games.clone());

    keep_alive();
    // GET /chat -> websocket upgrade
    let chat = warp::any()
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(users)
        .and(games)
        .map(|ws: warp::ws::Ws, users, games| {

            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, users, games))
        });


    warp::serve(chat)
        //.tls()
        //.cert_path("home/ing_hovind/certs/sjakkmotor.ikhovind.no/cert.pem")
        //.key_path("home/ing_hovind/certs/sjakkmotor.ikhovind.no/privkey.pem")
        .run(([0, 0, 0, 0], 3389)).await;
}

async fn user_connected(ws: WebSocket, users: Users, mut game: Games) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    log::info!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);


    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    log::error!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Save the sender in our list of connected users.
    users.write().await.insert(my_id, tx);
    game.insert(my_id, Board::from_fen(String::from("2kr3r/1b1p1pqp/4pN2/1Pb5/1p2P3/8/1PP1QPPP/R2R2K1 w - - 0 19")));

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        user_message(my_id, msg, &users,game.get_mut(&my_id).unwrap()).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &users, &mut game).await;
}

async fn user_message(my_id: usize, msg: Message, users: &Users, game: &mut Board) {
    // Skip any non-Text messages...
    match msg.to_str() {
        Ok(s) => {
            if s.len() > 5 {
                log::info!("received ping");
                return;
            }
            else {
                log::info!("Received message: {}", s);
                match Move::parse_move(&s, &game) {
                    Ok(m) => {
                        log::info!("Parsed move: {}", m);
                        game.make_move(&m);
                        let return_msg = match eval(&game) {
                            Some(ai_m) => {
                                log::info!("Making move: {}", ai_m);
                                game.make_move(&ai_m);
                                ai_m.to_string()
                            }
                            None => {
                                String::from("No available moves")
                            }
                        };
                        if let Err(_disconnected) = users.read().await.get(&my_id).unwrap().send(Message::text(return_msg)) {
                        }
                    }
                    Err(_) => {
                        log::warn!("could not parse move");
                        users.read().await.get(&my_id).unwrap().send(Message::text("Invalid move")).expect("Could not send message");
                    }
                }
            }
        }
        Err(_) => {
            log::warn!("Did not receive proper");
            return;
        }
    }
}

async fn user_disconnected(my_id: usize, users: &Users, games: &mut Games) {
    log::info!("user logged off: {}", my_id);
    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
    games.remove(&my_id);
}
