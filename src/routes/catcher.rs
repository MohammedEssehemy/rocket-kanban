use rocket::{http::Status, serde::json::Json, Request};

use super::http_error::HttpError;

#[rocket::catch(404)]
fn not_found(req: &Request) -> Json<HttpError> {
    let not_found_err = HttpError::not_found(&req.uri().to_string());
    Json(not_found_err)
}

#[rocket::catch(default)]
fn default_catcher<'r>(status: Status, req: &'r Request) -> Json<&'r HttpError> {
    let error = req.local_cache(|| {
        HttpError::from_message(&status, "something went wrong", &req.uri().to_string())
    });
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    rocket::catchers![not_found, default_catcher]
}
