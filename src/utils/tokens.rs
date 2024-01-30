use std::time::SystemTime;

use crate::{establish_connection, model::{Account, Token}, schema::{account_ranked_info::account_id, tokens::{dsl::tokens, valid_until}}};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, BelongingToDsl, QueryDsl, RunQueryDsl, dsl::now};
use rand::Rng;

pub fn generate_token(account: Account) {
    let tokens_query = Token::belonging_to(&account);
    let valid_tokens_query = tokens_query.filter(valid_until.gt(now));
    let valid_tokens = valid_tokens_query.load::<Token>(&mut establish_connection());

    loop {
        
    }
}