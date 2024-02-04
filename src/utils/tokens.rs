use std::time::SystemTime;

use crate::{establish_connection, model::{Account, NewToken, Token}, schema::{account::dsl::{account, id as account_id}, tokens::{dsl::{tokens, token}, valid_until}}};
use chrono::{Local, NaiveDateTime};
use diesel::{dsl::now, insert_into, BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

const LENGTH: usize = 25;

pub fn generate_token(acc: &Account) -> Option<String> {
    let generated_token = _generate_token();
    let new_token = NewToken {
        token: generated_token.to_owned(),
        account_id: acc.id,
        valid_until: Local::now().naive_local(),
    };

    if let Ok(_) = insert_into(tokens).values::<NewToken>(new_token).execute(&mut establish_connection()) {
        return Some(generated_token);
    }
    return None;
}

fn _generate_token() -> String {
    let rng = thread_rng();
    rng
        .sample_iter(&Alphanumeric)
        .take(LENGTH)
        .map(|c| c as char)
        .collect()
}

pub fn token_to_account(_token: &str) -> Option<(Account, Token)> {
    if let Some(token_obj) = tokens.filter(token.eq(_token)).load::<Token>(&mut establish_connection()).unwrap().first() {
        if let Some(acc) = account.filter(account_id.eq(token_obj.account_id)).load::<Account>(&mut establish_connection()).unwrap().first() {
            println!("Found account from token");
            return Some((acc.to_owned(), token_obj.to_owned()))
        }
    }
    println!("Cant find account from token");
    return None;
}