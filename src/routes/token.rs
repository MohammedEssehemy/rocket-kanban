use crate::db::{models::Token, DB};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};

use super::http_error::reply_with_err;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = &'r str;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // get request headers
        let headers = req.headers();

        // check that Authorization header exists
        let auth_header = headers.get_one("Authorization");
        if auth_header.is_none() {
            return reply_with_err(req, Status::Unauthorized, "missing Authorization header");
        }

        // and is well-formed
        let auth_header = auth_header.unwrap();
        let mut auth_header_parts = auth_header.split_ascii_whitespace();
        let auth_type = auth_header_parts.next();
        if auth_type.is_none() {
            return reply_with_err(req, Status::BadRequest, "malformed Authorization header");
        }

        // and uses the Bearer token authorization method
        let auth_type = auth_type.unwrap();
        if auth_type != "Bearer" {
            return reply_with_err(req, Status::Unauthorized, "invalid Authorization type");
        }

        // and the Bearer token is present
        let token_id = auth_header_parts.next();
        if token_id.is_none() {
            return reply_with_err(req, Status::Unauthorized, "missing Bearer token");
        }
        let token_id = token_id.unwrap();

        // we can use request.guard::<T>() to get a T from a request
        // which includes managed application state like our Db
        let db = req.guard::<&State<DB>>().await;

        let db = match db {
            Outcome::Success(db) => db,
            _ => return reply_with_err(req, Status::InternalServerError, "internal error"),
        };

        // validate token
        let token_result = db.validate_token(token_id);
        match token_result {
            Ok(token) => return Outcome::Success(token),
            Err(msg) => {
                eprintln!("{}", msg);
                return reply_with_err(
                    req,
                    Status::Unauthorized,
                    "invalid or expired Bearer token",
                );
            }
        };
    }
}
