#![feature(ascii_char)]

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use lib::database::model::Account;
use routes::matchmake::matchmake;
use routes::ping::ping;
use routes::{account::account_info, login::login, signup::signup};
use std::env;
use wrapper::auth_wrapper::Auth;

mod lib;
mod routes;
mod utils;
mod wrapper;

fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return MysqlConnection::establish(&database_url).expect("Could not connect to database");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .configure(|cfg| {
                // Configure Bearer token authentication for specific routes
                cfg.service(
                    web::scope("/api")
                        .wrap(Auth)
                        .service(account_info)
                        .service(ping)
                        .app_data(Data::new(Account::default())),
                );
            })  
            // Allow access to login and signup without Bearer token authentication
            .service(login)
            .service(signup)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}