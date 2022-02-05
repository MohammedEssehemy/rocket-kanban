#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

mod logger;
mod models;
mod schema;
mod db;
mod routes;

type StdErr = Box<dyn std::error::Error>;

#[rocket::main]
async fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    logger::init()?;

    rocket::build()
        .attach(db::KanbanDb::fairing())
        .register("/", routes::catchers())
        .mount("/api", routes::api())
        .launch()
        .await?;

    Ok(())

}
