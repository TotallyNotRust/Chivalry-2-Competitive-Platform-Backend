use crate::{
    establish_connection,
    lib::database::model::{Account, NewToken, Token},
    lib::database::schema::{
        account::dsl::{account, id as account_id},
        tokens::dsl::{id as token_id, invalidated, token, tokens},
    },
};
use chrono::{Duration, Local};
use diesel::{insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

const LENGTH: usize = 25;

pub fn generate_token(acc: &Account) -> Option<String> {
    println!("Generating new token");
    let generated_token = _generate_token();
    let new_token = NewToken {
        token: generated_token.to_owned(),
        account_id: acc.id,
        valid_until: Local::now().naive_utc() + Duration::minutes(60),
        invalidated: false,
    };

    if let Ok(_) = insert_into(tokens)
        .values::<NewToken>(new_token)
        .execute(&mut establish_connection())
    {
        return Some(generated_token);
    }
    return None;
}

fn _generate_token() -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(LENGTH)
        .map(|c| c as char)
        .collect()
}

fn _invalidate_tokens(_token: Token) {
    diesel::update(tokens)
        .set(invalidated.eq(true))
        .filter(token_id.eq(_token.id))
        .execute(&mut establish_connection())
        .unwrap();
}

pub fn token_to_account(_token: &str) -> Option<(Account, Token)> {
    if let Some(token_obj) = tokens
        .filter(token.eq(_token).and(invalidated.eq(false)))
        .load::<Token>(&mut establish_connection())
        .unwrap()
        .first()
    {
        if let Some(acc) = account
            .filter(account_id.eq(token_obj.account_id))
            .load::<Account>(&mut establish_connection())
            .unwrap()
            .first()
        {
            println!("Found account from token");
            return Some((acc.to_owned(), token_obj.to_owned()));
        }
    }
    println!("Cant find account from token");
    return None;
}
