use actix_web::web::Data;
use actix_web::{error, get, post, web, HttpResponse, Responder};
use bcrypt::verify;
use diesel::query_dsl::methods::FilterDsl;
use diesel::ExpressionMethods;
use diesel::{self, BoolExpressionMethods, Identifiable, RunQueryDsl};
use model::Account;
use serde::{Deserialize, Serialize};

use crate::schema::account::dsl::{account, email, salted_password, username};
use crate::schema::account::id;
use crate::{establish_connection, model, AccountData};

#[post("/account-info")]
pub async fn account_info(
    user: Data<Account>,
) -> Result<impl Responder, actix_web::error::Error> {
    println!("{:?}", user);
    return Ok(HttpResponse::Ok().body("Ok"));
}
