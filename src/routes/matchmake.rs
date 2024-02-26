use async_std::task::sleep;
use serde_derive::Deserialize;
use std::time::Duration;

use actix_web::web::{Data, Json};
use actix_web::{post, HttpMessage, HttpRequest, HttpResponse, Responder};
use lib::database::model::Account;

use crate::lib;

use super::account;

#[derive(Deserialize)]
pub struct GameMode {
    pub id: usize,
}

#[post("/matchmake")]
pub async fn matchmake(
    game_mode: Json<GameMode>,
    body: HttpRequest,
) -> Result<impl Responder, actix_web::error::Error> {
    let account = body.extensions().get::<Account>().unwrap().to_owned();
    println!(
        "account {:?} wants to queue gamemode {:?}",
        account.id, game_mode.id
    );

    register_queue(game_mode.id, account);

    return Ok(HttpResponse::Ok().body(format!("{}", game_mode.id)));
}

fn register_queue(gamemode: usize, account: Account) {}
