use actix_web::{error, get, post, web, Responder};
use bcrypt::verify;
use diesel::query_dsl::methods::FilterDsl;
use diesel::ExpressionMethods;
use diesel::{self, BoolExpressionMethods, Identifiable, RunQueryDsl};
use model::Account;
use serde::{Deserialize, Serialize};

use crate::schema::account::dsl::{account, email, salted_password, username};
use crate::schema::account::id;
use crate::utils::hashing::hash_password;
use crate::utils::tokens::generate_token;
use crate::{establish_connection, model};

#[derive(Deserialize, Debug)]
struct Login {
    identifier: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    auth: Auth,
    account: Account,
}

#[derive(Serialize, Debug)]
struct Auth {
    token: String,
}

#[post("/login")]
pub async fn login(login: web::Form<Login>) -> Result<impl Responder, actix_web::error::Error> {
    println!("{:?}", login);
    let identifier = &login.identifier.to_owned();

    let acc = account
        .filter(
            email
                .eq(identifier.to_owned())
                .or(username.eq(identifier.to_owned())),
        )
        .load::<Account>(&mut establish_connection());

    match acc {
        Ok(acc) => {
            if acc.len() > 0 {
                let actual_acc: &Account = &acc[0];

                match verify(&login.password, &actual_acc.salted_password.to_owned()) {
                    Ok(true) => {
                        if let Some(token) = generate_token(&actual_acc) {
                            let response = LoginResponse {
                                auth: Auth { token: token },
                                account: actual_acc.to_owned(),
                            };
                            return Ok(web::Json(response));
                        } else {
                            return Err(error::ErrorInternalServerError(
                                "Failed to generate token.",
                            ));
                        }
                    }
                    _ => {
                        println!("Invalid password");
                        return Err(error::ErrorUnauthorized::<String>(String::from(
                            "Invalid login",
                        )));
                    }
                }
            }
            return Err(error::ErrorUnauthorized::<String>(String::from(
                "Invalid login",
            )));
        }
        _ => {
            return Err(error::ErrorUnauthorized::<String>(String::from(
                "Invalid login",
            )))
        }
    }
}
