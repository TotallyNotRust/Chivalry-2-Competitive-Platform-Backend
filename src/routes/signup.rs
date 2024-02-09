use actix_web::{error, post, web, HttpResponse, Responder};
use diesel::query_dsl::methods::FilterDsl;
use diesel::ExpressionMethods;
use diesel::{self, insert_into, BoolExpressionMethods, RunQueryDsl};
use model::Account;
use serde::Deserialize;

use crate::model::NewAccount;
use crate::schema::account::dsl::{account, email, salted_password, username};
use crate::utils::hashing::hash_password;
use crate::{establish_connection, model};

#[derive(Deserialize, Debug)]
struct Login {
    username: String,
    email: String,
    password: String,
}

#[post("/signup")]
pub async fn signup(login: web::Form<Login>) -> Result<impl Responder, actix_web::error::Error> {
    println!("{:?}", login);

    let acc = account
        .filter(username.eq(&login.username).or(email.eq(&login.email)))
        .filter(salted_password.eq(&login.password))
        .load::<Account>(&mut establish_connection());

    match acc {
        Ok(acc) => {
            if acc.len() == 0 {
                let password= hash_password(&login.password);

                if password == None {
                    println!("failed to hash password");
                    return Err(error::ErrorInternalServerError("Failed to hash password"));
                }

                let new_acc = NewAccount {
                    username: login.username.to_owned(),
                    email: login.email.to_owned(),
                    salted_password: password.unwrap(),
                    account_closed: false,
                    punishment_id: None,
                };

                match insert_into(account).values::<NewAccount>(new_acc).execute(&mut establish_connection()) {
                    Ok(_) => return Ok(HttpResponse::Ok()),
                    Err(err) => {
                        println!("Got error while trying to create account: {}", err);
                        return Err(error::ErrorInternalServerError("Failed to create account"));
                    } 
                }
                 
            } else {
                return Err(error::ErrorConflict("Email already in use"));
            }
        }
        Err(err) => {
            println!("{:?}", err);
            return Err(error::ErrorInternalServerError("Internal server error"));
        }
    }
}
