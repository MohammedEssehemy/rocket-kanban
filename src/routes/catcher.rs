use rocket::{serde::json::Json, Request};

use super::http_error::HttpErrorResponse;

#[rocket::catch(404)]
fn not_found(req: &Request) -> Json<HttpErrorResponse> {
    let not_found_err = HttpErrorResponse::not_found(&req.uri().to_string());
    Json(not_found_err)
}

#[rocket::catch(default)]
fn default_catcher<'r>(req: &'r Request) -> Json<&'r HttpErrorResponse> {
    let default_error = HttpErrorResponse::server_error(&req.uri().to_string());
    let error = req.local_cache(|| default_error);
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    rocket::catchers![not_found, default_catcher]
}
