use rocket;

mod logger;
mod models;
mod schema;
mod db;
mod routes;

type StdErr = Box<dyn std::error::Error>;

#[rocket::launch]
fn rocket_main() -> _ {
    dotenv::dotenv().unwrap();
    logger::init().unwrap();
    
    let pool = db::KanbanDb::connect().unwrap();

    rocket::build()
        .manage(pool)
        .register("/", routes::catchers())
        .mount("/api", routes::api())
}
