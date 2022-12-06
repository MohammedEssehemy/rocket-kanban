use super::http_error::HttpError;
use crate::db::DbResult;
use rocket::{
    async_trait,
    response::{Responder, Result},
    serde::json::Json,
    Request,
};
use serde::Serialize;

pub enum RouteResult<T: Serialize> {
    Ok(T),
    Err(HttpError),
}

#[async_trait]
impl<'r, T: Serialize> Responder<'r, 'static> for &'r RouteResult<T> {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        match self {
            RouteResult::Ok(res) => Json(res).respond_to(req),
            RouteResult::Err(err) => err.respond_to(req),
        }
    }
}

#[async_trait]
impl<'r, T: Serialize> Responder<'r, 'static> for RouteResult<T> {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        (&self).respond_to(req)
    }
}

impl<T: Serialize> From<DbResult<T>> for RouteResult<T> {
    fn from(db_result: DbResult<T>) -> Self {
        db_result.map_or_else(
            |err| RouteResult::Err(err.into()),
            |res| RouteResult::Ok(res),
        )
    }
}
