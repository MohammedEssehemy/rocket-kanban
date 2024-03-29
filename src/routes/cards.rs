use super::http_response::RouteResult;
use crate::db::{
    models::{Card, CreateCardDTO, Token, UpdateCardDTO},
    DB,
};
use rocket::{delete, get, patch, post, routes, serde::json::Json, Route, State};

// card routes
#[get("/boards/<board_id>/cards")]
async fn cards(_t: &Token, db: &State<DB>, board_id: i64) -> RouteResult<Vec<Card>> {
    db.cards(board_id).into()
}

#[post("/cards", data = "<create_card>")]
async fn create_card(
    _t: &Token,
    db: &State<DB>,
    create_card: Json<CreateCardDTO>,
) -> RouteResult<Card> {
    db.create_card(create_card.0).into()
}

#[patch("/cards/<card_id>", data = "<update_card>")]
async fn update_card(
    _t: &Token,
    db: &State<DB>,
    card_id: i64,
    update_card: Json<UpdateCardDTO>,
) -> RouteResult<Card> {
    db.update_card(card_id, update_card.0).into()
}

#[delete("/cards/<card_id>")]
async fn delete_card(_t: &Token, db: &State<DB>, card_id: i64) -> RouteResult<()> {
    db.delete_card(card_id).into()
}

pub fn api() -> Vec<Route> {
    routes![cards, create_card, update_card, delete_card]
}
