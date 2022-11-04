use rocket::{catch, catchers, http::Status, Catcher, Request};

use super::http_error::HttpError;

#[catch(404)]
fn not_found(req: &Request) -> HttpError {
    HttpError::not_found(&req.uri().to_string())
}

#[catch(default)]
fn default_catcher<'r>(status: Status, req: &'r Request) -> &'r HttpError {
    req.local_cache(|| {
        HttpError::from_message(&status, "something went wrong", &req.uri().to_string())
    })
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, default_catcher]
}
