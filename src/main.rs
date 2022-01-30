#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

mod logger;
mod models;
mod schema;
mod db;
mod routes;

type StdErr = Box<dyn std::error::Error>;

#[get("/")]
fn hello_world() -> &'static str {
   "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    logger::init()?;
    let db = db::Db::connect()?;

    rocket::build()
        .manage(db)
        .mount("/", rocket::routes![hello_world])
        .mount("/api", routes::api())
        .launch()
        .await?;

    Ok(())

}
