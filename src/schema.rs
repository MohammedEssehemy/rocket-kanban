// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status_enum"))]
    pub struct StatusEnum;
}

diesel::table! {
    boards (id) {
        id -> Int8,
        name -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusEnum;

    cards (id) {
        id -> Int8,
        board_id -> Int8,
        description -> Text,
        status -> StatusEnum,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tokens (id) {
        id -> Text,
        expired_at -> Timestamptz,
    }
}

diesel::joinable!(cards -> boards (board_id));

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    cards,
    tokens,
);
