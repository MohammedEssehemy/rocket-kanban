use dotenv::dotenv;
use rocket::{build, launch, Build, Rocket};
mod db;
mod logger;
mod routes;
use db::DB;
use routes::{api, catchers, Auth};

#[launch]
fn rocket_main() -> Rocket<Build> {
    dotenv().unwrap();
    logger::init();

    let db_connection = DB::connect().unwrap();

    build()
        .manage(db_connection)
        .attach(Auth)
        .register("/", catchers())
        .mount("/api", api())
}
