use super::http_response::RouteResult;
use crate::db::{
    models::{Board, BoardSummary, CreateBoardDTO, Token},
    DB,
};
use rocket::{delete, get, post, routes, serde::json::Json, Route, State};

// board routes
#[get("/boards")]
async fn boards(_t: &Token, db: &State<DB>) -> RouteResult<Vec<Board>> {
    db.boards().into()
}

#[post("/boards", format = "json", data = "<create_board>")]
async fn create_board(
    _t: &Token,
    db: &State<DB>,
    create_board: Json<CreateBoardDTO>,
) -> RouteResult<Board> {
    db.create_board(create_board.0).into()
}

#[get("/boards/<board_id>/summary")]
async fn board_summary(_t: &Token, db: &State<DB>, board_id: i64) -> RouteResult<BoardSummary> {
    db.board_summary(board_id).into()
}

#[delete("/boards/<board_id>")]
async fn delete_board(_t: &Token, db: &State<DB>, board_id: i64) -> RouteResult<()> {
    db.delete_board(board_id).into()
}

pub fn api() -> Vec<Route> {
    routes![boards, create_board, board_summary, delete_board]
}
