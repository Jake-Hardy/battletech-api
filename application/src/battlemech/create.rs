use domain::battlemech_model::Battlemech;
use shared::response_models::{Response, ResponseBody};

use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_battlemech(battlemech: Json<Battlemech>) -> Created<String> {
	use domain::mongodb_repo::MongoRepo;

	let db = MongoRepo::init();

	let data = Battlemech {
		id: None,
		name: battlemech.name.to_owned(),
		designation: battlemech.designation.to_owned(),
		components: battlemech.components.clone(),
	};
	let battlemech_detail = db.create_battlemech(data);
	match battlemech_detail {
		Ok(battlemech) => {
			let response = Response { 
				body: ResponseBody::Message(format!("Battlemech created: {}", battlemech.inserted_id.to_string())),
			};
			Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
		},
		Err(err) => panic!("Database error - {}", err)
	}
}