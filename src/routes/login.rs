use actix_web::{get, post, web, Responder, error};
use bcrypt::verify;
use diesel::{self, BoolExpressionMethods, Identifiable, RunQueryDsl};
use diesel::query_dsl::methods::FilterDsl;
use diesel::ExpressionMethods;
use serde::{Deserialize, Serialize};
use model::Account;

use crate::schema::account::id;
use crate::utils::hashing::hash_password;
use crate::{establish_connection, model};
use crate::schema::{account::dsl::{account, username, salted_password, email}};

#[derive(Deserialize, Debug)]
struct Login {
    identifier: String,
    password: String,
}

#[post("/login")]
pub async fn login(login: web::Form<Login>) -> Result<impl Responder, actix_web::error::Error> {

    println!("{:?}", login);
    let identifier = &login.identifier.to_owned();

    let acc = account
    .filter(email.eq(identifier.to_owned()).or(username.eq(identifier.to_owned())))
    .load::<Account>(&mut establish_connection());

    match acc {
        Ok(acc) => {
            if acc.len() > 0 {
                let actual_acc: &Account = &acc[0];

                match verify(&login.password, &actual_acc.salted_password.to_owned()) {
                    Ok(true) => return Ok(web::Json(actual_acc.to_owned())),
                    _ => {
                        println!("Invalid password");
                        return Err(error::ErrorUnauthorized::<String>(String::from("Invalid login")));
                    },
                }   
            }
            return Err(error::ErrorUnauthorized::<String>(String::from("Invalid login")));
        },
        _ => return Err(error::ErrorUnauthorized::<String>(String::from("Invalid login"))),
    }

}