use super::http_error::HttpError;
use crate::db::{models::Token, DB};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Status,
    request::{FromRequest, Outcome},
    Data, Request, State,
};

enum AuthError {
    EmptyAuth,
    Malformed,
    InvalidType,
    MissingBearerToken,
    InternalError,
    InvalidToken,
}

impl ToString for AuthError {
    fn to_string(&self) -> String {
        match self {
            Self::EmptyAuth => String::from("missing Authorization header"),
            Self::Malformed => String::from("malformed Authorization header"),
            Self::InvalidType => String::from("invalid Authorization type"),
            Self::MissingBearerToken => String::from("missing Bearer token"),
            Self::InternalError => String::from("internal error"),
            Self::InvalidToken => String::from("invalid or expired Bearer token"),
        }
    }
}

type AuthResult = Result<Token, AuthError>;

pub struct Auth;

#[rocket::async_trait]
impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "Request Token",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        // check that Authorization header exists
        let auth_header = req.headers().get_one("Authorization");
        if auth_header.is_none() {
            req.local_cache(|| AuthResult::Err(AuthError::EmptyAuth));
            return;
        }
        let auth_header = auth_header.unwrap();

        // and is well-formed
        let mut auth_header_parts = auth_header.split_ascii_whitespace();
        let auth_type = auth_header_parts.next();
        if auth_type.is_none() {
            req.local_cache(|| AuthResult::Err(AuthError::Malformed));
            return;
        }

        // and uses the Bearer token authorization method
        let auth_type = auth_type.unwrap();
        if auth_type != "Bearer" {
            req.local_cache(|| AuthResult::Err(AuthError::InvalidType));
            return;
        }

        // and the Bearer token is present
        let token_id = auth_header_parts.next();
        if token_id.is_none() {
            req.local_cache(|| AuthResult::Err(AuthError::MissingBearerToken));
            return;
        }
        let token_id = token_id.unwrap();

        // we can use request.guard::<T>() to get a T from a request
        // which includes managed application state like our Db
        let db = req.guard::<&State<DB>>().await.succeeded();
        if db.is_none() {
            req.local_cache(|| AuthResult::Err(AuthError::InternalError));
            return;
        }
        let db = db.unwrap();

        // validate token
        match db.validate_token(token_id) {
            Ok(token) => {
                req.local_cache(|| AuthResult::Ok(token));
            }
            Err(msg) => {
                eprintln!("{}", msg);
                req.local_cache(|| AuthResult::Err(AuthError::InvalidToken));
            }
        };
        return;
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Token {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // AuthResult should have been resolved from the fairing
        let auth_result = req.local_cache(|| AuthResult::Err(AuthError::InternalError));

        match auth_result {
            AuthResult::Ok(token) => Outcome::Success(token),
            AuthResult::Err(auth_error) => {
                req.local_cache(|| {
                    HttpError::from_message(
                        &Status::Unauthorized,
                        &auth_error.to_string(),
                        &req.uri().to_string(),
                    )
                });
                Outcome::Failure((Status::Unauthorized, auth_error.to_string()))
            }
        }
    }
}
