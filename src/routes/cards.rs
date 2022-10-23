use super::http_error::StdErr;
use crate::db::{
    models::{Card, CreateCardDTO, Token, UpdateCardDTO},
    DB,
};
use rocket::{delete, get, patch, post, response::Debug, routes, serde::json::Json, Route, State};

// card routes
#[get("/boards/<board_id>/cards")]
async fn cards(_t: Token, db: &State<DB>, board_id: i64) -> Result<Json<Vec<Card>>, Debug<StdErr>> {
    db.cards(board_id).map(Json).map_err(Debug)
}

#[post("/cards", data = "<create_card>")]
async fn create_card(
    _t: Token,
    db: &State<DB>,
    create_card: Json<CreateCardDTO>,
) -> Result<Json<Card>, Debug<StdErr>> {
    db.create_card(create_card.0).map(Json).map_err(Debug)
}

#[patch("/cards/<card_id>", data = "<update_card>")]
async fn update_card(
    _t: Token,
    db: &State<DB>,
    card_id: i64,
    update_card: Json<UpdateCardDTO>,
) -> Result<Json<Card>, Debug<StdErr>> {
    db.update_card(card_id, update_card.0)
        .map(Json)
        .map_err(Debug)
}

#[delete("/cards/<card_id>")]
async fn delete_card(_t: Token, db: &State<DB>, card_id: i64) -> Result<(), Debug<StdErr>> {
    db.delete_card(card_id).map_err(Debug)
}

pub fn api() -> Vec<Route> {
    routes![cards, create_card, update_card, delete_card]
}
