use rocket::Route;
mod auth;
mod boards;
mod cards;
mod catcher;
mod http_error;

pub use auth::Auth;
pub use catcher::catchers;

pub fn api() -> Vec<Route> {
    let mut apis = vec![];
    apis.append(&mut boards::api());
    apis.append(&mut cards::api());
    apis
}
