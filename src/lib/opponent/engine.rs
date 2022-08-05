use std::cmp::min;
use std::sync::atomic::AtomicI16;
use std::sync::{Arc, mpsc, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::log;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;
use warp::path::end;
use crate::{Board};
use crate::consts::board_consts::{N_INF, P_INF};
use crate::mv::Move;
use crate::opponent::game_stage::GameStage::EARLY;
use crate::opponent::move_ordering::order_moves;
use crate::opponent::search::search_moves;
use crate::opponent::static_eval::{eval_pos, weight_king_pos};


pub fn eval(mut b: Board, depth: u8) -> Option<Move> {
    log::info!("thinking about move");
    const NUM_THREADS: usize = 4;
    let mut moves = b.get_all_moves();
    let mut handles = vec![];
    order_moves(&b, &mut moves);
    let mut moves = Arc::new(moves);
    let len = moves.len();
    if len > 0 {
        let chunk_size = (len + NUM_THREADS - 1) / NUM_THREADS; // divide by threads rounded up.
        for j in 0..NUM_THREADS {
            let start = j * chunk_size;
            let end = min(start + chunk_size, len);
            let conn = moves.clone();
            let t =  thread::spawn(move || {
                let mut best_score = i16::MIN;
                let mut best_yet = Move::new_move(0,0,false);
                for k in start..end {
                    let mv = conn.get(k).unwrap();
                    log::info!("Evaluating: {}", mv);
                    let curr = -search_moves(b.make_move(&mv), depth, N_INF, P_INF, &EARLY);
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