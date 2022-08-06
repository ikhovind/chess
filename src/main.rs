use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use warp::sse::keep_alive;
use warp::ws::{Message, WebSocket};

use shellfishlib::consts::position_consts::BASE_POS;
use shellfishlib::mv::Move;
use shellfishlib::opponent::engine::eval;
use shellfishlib::opponent::game::Game;
use shellfishlib::opponent::game_stage::GameStage::EARLY;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;
type Games = HashMap<usize, Game>;

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
        /*
        .tls()
        .cert_path("home/ing_hovind/certs/sjakkmotor.ikhovind.no/cert.pem")
        .key_path("home/ing_hovind/certs/sjakkmotor.ikhovind.no/privkey.pem")

         */
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
    game.insert(my_id, Game::from_fen(BASE_POS));

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

async fn user_message(my_id: usize, msg: Message, users: &Users, game: &mut Game) {
    // Skip any non-Text messages...
    match msg.to_str() {
        Ok(s) => {
            if s.len() > 5 {
                log::info!("received ping");
                return;
            }
            else {
                match Move::parse_move(&s, &game.board) {
                    Ok(m) => {
                        log::info!("Received move: {}", m);
                        game.board = game.board.make_move(&m);
                        if game.stage == EARLY {
                            game.history.push_str(&m.to_string());
                        }
                        let return_msg = match eval(game, 4) {
                            Some(ai_m) => {
                                log::info!("Making move: {}", ai_m);
                                game.board = game.board.make_move(&ai_m);
                                if game.stage == EARLY {
                                    game.history.push_str(&ai_m.to_string());
                                }
                                game.set_stage();
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
