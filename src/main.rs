use dotenv::dotenv;
use rocket::{build, launch, Build, Rocket};
mod db;
mod logger;
mod routes;
use db::DB;

#[launch]
fn rocket_main() -> Rocket<Build> {
    dotenv().unwrap();
    logger::init();

    let db_connection = DB::connect().unwrap();

    build()
        .manage(db_connection)
        .register("/", routes::catchers())
        .mount("/api", routes::api())
}
