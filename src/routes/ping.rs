use actix_web::web::Data;
use actix_web::{post, HttpResponse, Responder};
use lib::database::model::Account;

use crate::lib;

#[post("/ping")]
pub async fn ping() -> Result<impl Responder, actix_web::error::Error> {
    return Ok(HttpResponse::Ok().body("Ok"));
}
