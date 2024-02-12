use async_std::task::sleep;
use std::time::Duration;

use actix_web::web::Data;
use actix_web::{post, HttpMessage, HttpRequest, HttpResponse, Responder};
use model::Account;

use crate::model;

struct GameMode {
    pub id: i32,
}

#[post("/matchmake")]
pub async fn matchmake(
    game_mode: Data<GameMode>,
) -> Result<impl Responder, actix_web::error::Error> {
    println!(
        "body {:?}, gamemode {:?}",
        1, //body.extensions().get::<Account>(),
        game_mode.id
    );
    sleep(Duration::from_secs(5)).await;
    println!("Responding");
    return Ok(HttpResponse::Ok().body("Ok"));
}
