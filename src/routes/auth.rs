use super::http_error::HttpError;
use crate::db::{models::Token, DB};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Status,
    request::{FromRequest, Outcome},
    Data, Request, State,
};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Token {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let url = req.uri().to_string();
        let auth_result = req.local_cache(|| AuthResult::Err(AuthError::InternalError));

        match auth_result {
            AuthResult::Ok(token) => Outcome::Success(token),
            AuthResult::Err(auth_error) => {
                req.local_cache(|| {
                    HttpError::from_message(&Status::Unauthorized, &auth_error.to_string(), &url)
                });
                Outcome::Failure((Status::Unauthorized, auth_error.to_string()))
            }
        }
    }
}

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

enum AuthResult {
    Ok(Token),
    Err(AuthError),
}

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
        let headers = req.headers();
        // check that Authorization header exists
        let auth_header = headers.get_one("Authorization");
        if auth_header.is_none() {
            req.local_cache(|| AuthResult::Err(AuthError::EmptyAuth));
            return;
        }

        // and is well-formed
        let auth_header = auth_header.unwrap();
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
        let db = req.guard::<&State<DB>>().await;

        let db = match db {
            Outcome::Success(db) => db,
            _ => {
                req.local_cache(|| AuthResult::Err(AuthError::InternalError));
                return;
            }
        };

        // validate token
        let token_result = db.validate_token(token_id);
        match token_result {
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
