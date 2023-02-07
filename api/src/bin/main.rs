#[macro_use] extern crate rocket;
use api::battlemech_handler;

#[launch]
fn rocket() -> _ {
    // let db = MongoRepo::init();
    rocket::build()
        // .manage(db)
        .mount("/api", routes![
			battlemech_handler::list_battlemechs_handler,
			battlemech_handler::list_battlemech_handler,
			battlemech_handler::create_battlemech_handler,
			battlemech_handler::update_battlemech_handler,
			battlemech_handler::delete_battlemech_handler,
		])
}