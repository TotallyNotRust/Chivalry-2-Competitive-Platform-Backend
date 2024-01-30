use std::time::SystemTime;

use diesel::{
    prelude::Insertable, Associations, Identifiable, Queryable
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::schema::*;

#[derive(Queryable, Insertable, Identifiable, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[diesel(table_name = account)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub salted_password: String,
    pub account_closed: bool,
    pub punishment_id: Option<i32>,
}
#[derive(Queryable, Insertable, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub username: String,
    pub email: String,
    pub salted_password: String,
    pub account_closed: bool,
    pub punishment_id: Option<i32>,
}

#[derive(Associations, Queryable, Insertable, Identifiable, Debug, PartialEq, Clone, Deserialize, Serialize)]
#[diesel(table_name = tokens)]
#[diesel(belongs_to(Account))]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub account_id: i32,
    pub valid_until: NaiveDateTime
}
