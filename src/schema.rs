// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        #[max_length = 25]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 60]
        salted_password -> Varchar,
        account_closed -> Bool,
        punishment_id -> Nullable<Integer>,
    }
}

diesel::table! {
    account_match (id) {
        id -> Bigint,
        match_id -> Bigint,
        account_id -> Integer,
    }
}

diesel::table! {
    account_ranked_info (id) {
        id -> Integer,
        account_id -> Integer,
        standing -> Integer,
    }
}

diesel::table! {
    c2_match (id) {
        id -> Bigint,
        gamemode -> Smallint,
    }
}

diesel::table! {
    punishment (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Tinyint,
        end_date -> Nullable<Date>,
        #[max_length = 16]
        evidence_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    queue (id) {
        id -> Bigint,
        gamemode -> Smallint,
        account_id -> Integer,
    }
}

diesel::table! {
    tokens (id) {
        id -> Integer,
        #[max_length = 25]
        token -> Varchar,
        account_id -> Integer,
        valid_until -> Timestamp,
        invalidated -> Bool,
    }
}

diesel::joinable!(account -> punishment (punishment_id));
diesel::joinable!(account_match -> account (account_id));
diesel::joinable!(account_match -> c2_match (match_id));
diesel::joinable!(account_ranked_info -> account (account_id));
diesel::joinable!(queue -> account (account_id));
diesel::joinable!(tokens -> account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    account_match,
    account_ranked_info,
    c2_match,
    punishment,
    queue,
    tokens,
);
