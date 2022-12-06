use rocket::{catch, catchers, http::Status, Catcher, Request};

use super::http_error::HttpError;

#[catch(404)]
fn not_found(_req: &Request) -> HttpError {
    HttpError::not_found()
}

#[catch(default)]
fn default_catcher<'r>(_status: Status, req: &'r Request) -> &'r HttpError {
    req.local_cache(|| HttpError::internal(None))
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, default_catcher]
}
