use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{http, State};
use uuid::Uuid;

use crate::db::{
    models::{
        Board, 
        BoardSummary, 
        Card, 
        CreateBoardDTO, 
        CreateCardDTO, 
        ErrorResponse, 
        Token,
        UpdateCardDTO,
    },
    DB,
};

type StdErr = Box<dyn std::error::Error>;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = &'static str;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // get request headers
        let headers = req.headers();

        // check that Authorization header exists
        let maybe_auth_header = headers.get_one("Authorization");
        if maybe_auth_header.is_none() {
            return Outcome::Failure((http::Status::Unauthorized, "missing Authorization header"));
        }

        // and is well-formed
        let auth_header = maybe_auth_header.unwrap();
        let mut auth_header_parts = auth_header.split_ascii_whitespace();
        let maybe_auth_type = auth_header_parts.next();
        if maybe_auth_type.is_none() {
            return Outcome::Failure((
                http::Status::Unauthorized,
                "malformed Authorization header",
            ));
        }

        // and uses the Bearer token authorization method
        let auth_type = maybe_auth_type.unwrap();
        if auth_type != "Bearer" {
            return Outcome::Failure((http::Status::BadRequest, "invalid Authorization type"));
        }

        // and the Bearer token is present
        let maybe_token_id = auth_header_parts.next();
        if maybe_token_id.is_none() {
            return Outcome::Failure((http::Status::Unauthorized, "missing Bearer token"));
        }
        let token_id = maybe_token_id.unwrap();

        // we can use request.guard::<T>() to get a T from a request
        // which includes managed application state like our Db
        let outcome_db = req.guard::<&State<DB>>().await;
        let db = match outcome_db {
            Outcome::Success(db) => db,
            _ => return Outcome::Failure((http::Status::InternalServerError, "internal error")),
        };

        // validate token
        let token_result = db.validate_token(token_id.to_string());
        match token_result {
            Ok(token) => Outcome::Success(token),
            Err(msg) => {
                eprintln!("{}", msg);
                Outcome::Failure((
                    http::Status::Unauthorized,
                    "invalid or expired Bearer token",
                ))
            }
        }
    }
}

// board routes

#[rocket::get("/boards")]
async fn boards(_t: Token, db: &State<DB>) -> Result<Json<Vec<Board>>, Debug<StdErr>> {
    db.boards().map(Json).map_err(Debug)
}

#[rocket::post("/boards", data = "<create_board>")]
async fn create_board(
    _t: Token,
    db: &State<DB>,
    create_board: Json<CreateBoardDTO>,
) -> Result<Json<Board>, Debug<StdErr>> {
    db.create_board(create_board.0).map(Json).map_err(Debug)
}

#[rocket::get("/boards/<board_id>/summary")]
async fn board_summary(
    _t: Token,
    db: &State<DB>,
    board_id: i64,
) -> Result<Json<BoardSummary>, Debug<StdErr>> {
    db.board_summary(board_id).map(Json).map_err(Debug)
}

#[rocket::delete("/boards/<board_id>")]
async fn delete_board(_t: Token, db: &State<DB>, board_id: i64) -> Result<(), Debug<StdErr>> {
    db.delete_board(board_id).map_err(Debug)
}

// card routes

#[rocket::get("/boards/<board_id>/cards")]
async fn cards(_t: Token, db: &State<DB>, board_id: i64) -> Result<Json<Vec<Card>>, Debug<StdErr>> {
    db.cards(board_id).map(Json).map_err(Debug)
}

#[rocket::post("/cards", data = "<create_card>")]
async fn create_card(
    _t: Token,
    db: &State<DB>,
    create_card: Json<CreateCardDTO>,
) -> Result<Json<Card>, Debug<StdErr>> {
    db.create_card(create_card.0).map(Json).map_err(Debug)
}

#[rocket::patch("/cards/<card_id>", data = "<update_card>")]
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

#[rocket::delete("/cards/<card_id>")]
async fn delete_card(_t: Token, db: &State<DB>, card_id: i64) -> Result<(), Debug<StdErr>> {
    db.delete_card(card_id).map_err(Debug)
}

#[rocket::catch(404)]
fn not_found(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code: "NOT_FOUND".to_string(),
        message: "route not found".to_string(),
        url: req.uri().to_string(),
        x_trace_id: Uuid::new_v4(),
    })
}

pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
        boards,
        create_board,
        board_summary,
        delete_board,
        cards,
        create_card,
        update_card,
        delete_card,
    ]
}

pub fn catchers() -> Vec<rocket::Catcher> {
    rocket::catchers![not_found,]
}
