use rocket::{
    response::{Debug, Responder},
    serde::json::Json,
};

use crate::db::{DbErr, DbResult};

#[derive(Responder)]
pub enum RouteResult<T> {
    #[response(status = 200, content_type = "json")]
    Success(Json<T>),
    #[response(status = 500, content_type = "json")]
    Fail(Debug<DbErr>),
}

impl<T> From<DbResult<T>> for RouteResult<T> {
    fn from(db_result: DbResult<T>) -> Self {
        db_result.map_or_else(
            // fail
            |err| RouteResult::Fail(Debug(err)),
            // success
            |res| RouteResult::Success(Json(res)),
        )
    }
}
