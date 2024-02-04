use crate::utils::tokens::{generate_token, token_to_account};
use actix_cors::Cors;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder};
use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use model::Account;
use routes::account;
use routes::{account::account_info, login::login, signup::signup};
use std::env;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};

mod model;
mod routes;
mod schema;
mod utils;

fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return MysqlConnection::establish(&database_url).expect("Could not connect to database");
}

struct AccountData(Account);

async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    eprintln!("{credentials:?}");
    if let Some((acc, token_obj)) = token_to_account(credentials.token()) {

        if token_obj.is_expired() {
            // Generate token automatically saves the new token, so we just need to check if an error occured (None is returned)
            if generate_token(&acc).is_none() {
                eprintln!("Failed to generate new token for account with email: {:?}", &acc.email)
            }
        }

        req.extensions_mut().insert(Data::new(acc));

        return Ok(req);
    } else {
        println!("Unauthorized");
        return Err((ErrorUnauthorized("Please login"), req));
    }
}

// Middleware function to extract, validate, and set a new token in the response headers
async fn validate_and_set_token(
    req: HttpRequest,
    auth: BearerAuth,
    identity: Identity,
) -> Result<HttpResponseBuilder, HttpResponse> {
    let token = auth.token();
    
    // Replace "your_custom_secret" with your actual secret
    let expected_token = "your_custom_token";

    // Validate the token
    if token == expected_token {
        // Token is valid, set a dummy user ID in the Identity
        identity.set("dummy_user_id");

        // Set a new token in the response headers
        let new_token = "new_custom_token";
        let resp = HttpResponse::Ok();
        resp.append_header(("New-Authorization", format!("Bearer {:?}", new_token)).finish());
        Ok(resp)
    } else {
        // Token is invalid
        Err(HttpResponse::Unauthorized().body("Invalid token"))
    }
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
                        .wrap(HttpAuthentication::bearer(ok_validator))
                        .service(account_info)
                        .app_data(Data::new(Account::default())),
                );
            })
            // Allow access to login and signup without Bearer token authentication
            .service(login)
            .service(signup)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
