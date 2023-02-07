use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
	sync::{Client, Database},
};

pub fn establish_connection() -> Database {
	dotenv().ok();
	let uri = match env::var("MONGOURI") {
		Ok(v) => v.to_string(),
		Err(_) => format!("Error loading env variable"),
	};
	let client = Client::with_uri_str(uri).unwrap();

	client.database("rustDB")
}