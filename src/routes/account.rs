use actix_web::web::Data;
use actix_web::{post, HttpResponse, Responder};
use lib::database::model::Account;

use crate::lib;

#[post("/account-info")]
pub async fn account_info(user: Data<Account>) -> Result<impl Responder, actix_web::error::Error> {
    println!("{:?}", user);
    return Ok(HttpResponse::Ok().body("Ok"));
}
