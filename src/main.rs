mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::battlemech_api::{create_battlemech, get_all_battlemechs, get_battlemech, update_battlemech, delete_battlemech};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_battlemech])
        .mount("/", routes![get_all_battlemechs])
        .mount("/", routes![get_battlemech])
        .mount("/", routes![update_battlemech])
        .mount("/", routes![delete_battlemech])
}
