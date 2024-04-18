#[macro_use] extern crate rocket;


use shellfishlib::opponent::engine::eval;
use shellfishlib::opponent::game::Game;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize,Serialize};
use rocket::response::content::RawJson;

#[derive(Deserialize)]
struct BestMoveReq {
    fen: String,
}


#[derive(Serialize)]
struct Response {
    response: String,
    fen: String
}



#[post("/", data = "<task>")]
fn get_best_move(task: Json<BestMoveReq>) -> Json<Response> {
    let mut game = Game::from_fen(task.fen.as_str());
    let return_msg = match eval(&mut game, 4) {
        Some(ai_m) => {
            log::info!("Making move: {}", ai_m);
            ai_m.to_string()
        }
        None => {
            String::from("No available moves")
        }
    };
    let res = Response {
        response:return_msg,
        fen: task.fen.clone()
    };
    Json(res)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/chess", routes![get_best_move])
}
