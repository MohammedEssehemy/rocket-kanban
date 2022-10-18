mod logger;
mod db;
mod routes;

use rocket;
use db::DB;

#[rocket::launch]
fn rocket_main() -> _ {
    dotenv::dotenv().unwrap();
    logger::init();
    
    let db_connection = DB::connect().unwrap();

    rocket::build()
        .manage(db_connection)
        .register("/", routes::catchers())
        .mount("/api", routes::api())
}
