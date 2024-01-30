use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpServer};
use diesel::{Connection, MysqlConnection};
use routes::{login::login, signup::signup};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use dotenv::dotenv;
use std::env;

mod routes;
mod model;
mod schema;
mod utils;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    return MysqlConnection::establish(&database_url).expect("Could not connect to database");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(signup)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}