#[macro_use] extern crate rocket;


use shellfishlib::opponent::engine::eval;
use shellfishlib::opponent::game::Game;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize,Serialize};
use rocket::response::content::RawJson;


use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "*",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Deserialize)]
struct BestMoveReq {
    fen: String,
}


#[derive(Serialize)]
struct BestMoveResponse {
    response: String,
    fen: String
}



#[post("/", data = "<task>")]
fn get_best_move(task: Json<BestMoveReq>) -> Json<BestMoveResponse> {
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
    let res = BestMoveResponse {
        response:return_msg,
        fen: task.fen.clone()
    };
    Json(res)
}


#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS)
        .mount("/chess", routes![get_best_move])
}
