use std::cmp::min;
use std::env::var;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, mpsc, RwLock};
use std::thread;
use rand::Rng;
use log::log;
use crate::{Board};
use crate::consts::board_consts::{N_INF, P_INF};
use crate::mv::Move;
use crate::opponent::game::Game;
use crate::opponent::game_stage::GameStage;
use crate::opponent::game_stage::GameStage::{EARLY, MIDDLE};
use crate::opponent::move_ordering::order_moves;
use crate::opponent::search::search_moves;


pub fn eval(g: &mut Game, depth: u8) -> Option<Move> {
    log::info!("thinking about move");
    const NUM_THREADS: usize = 4;
    let mut moves = g.board.get_all_moves();
    let mut handles = vec![];
    if g.stage == EARLY {
        match search_for_move(String::from(g.clone().history), &g.board) {
            None => {
                log::info!("found no matching move");
                g.stage = MIDDLE;
            }
            Some(m) => {
                log::info!("Found move from book: {}", m);
                return Some(m);
            }
        }
    }

    g.set_stage();
    log::info!("Game stage is: {:?}", g.stage);
    order_moves(&g.board, &mut moves);
    let mut moves = Arc::new(moves);
    let len = moves.len();
    if len > 0 {
        let chunk_size = (len + NUM_THREADS - 1) / NUM_THREADS; // divide by threads rounded up.
        for j in 0..NUM_THREADS {
            let start = j * chunk_size;
            let end = min(start + chunk_size, len);
            let conn = moves.clone();
            let local_game = g.clone();
            let t =  thread::spawn(move || {
                let mut best_score = i16::MIN;
                let mut best_yet = Move::new_move(0,0,false);
                for k in start..end {
                    let mv = conn.get(k).unwrap();
                    log::info!("Evaluating: {}", mv);
                    let curr = -search_moves(local_game.board.make_move(&mv), depth, N_INF, P_INF, local_game.stage);
                    if curr > best_score {
                        log::info!("new best move found");
                        best_score = curr;
                        best_yet = *mv;
                    }
                }
                return (best_score, best_yet);
            });
            handles.push(t);
        }
        let mut best_score = i16::MIN;
        let mut best_yet = moves[0];
        for handle in handles {
            let res = handle.join().unwrap();
            if res.0 > best_score {
                best_score = res.0;
                best_yet = res.1;
            }
        }
        log::info!("Returning best move with score: {}", best_score);
        return Some(best_yet);
    }
    return None;
}

fn search_for_move(opening: String, b: &Board) -> Option<Move>{
    log::info!("searching for opening with move: {}", opening);
    let file = File::open("resources/book.pgn").unwrap();
    let reader = BufReader::new(file);
    let mut possible: Vec<String> = vec![];
    if opening.len() == 0 {
        for line in reader.lines() {
            possible.push(line.unwrap());
        }
    }
    else {
        for line in reader.lines() {
            if line.as_ref().unwrap().starts_with(&opening) {
                possible.push(line.unwrap());
            }
        }
    }
    return if possible.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let line = &possible[rng.gen_range(0..possible.len())][opening.len()..opening.len() + 4];
        Some(Move::parse_move(line, b).unwrap())
    }
}
