use domain::battlemech_model::Battlemech;
use domain::component_model::Component;
use shared::response_models::{Response, ResponseBody};

use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_battlemech(battlemech: Json<Battlemech>) -> Result<Battlemech, String> {
	use domain::mongodb_repo::MongoRepo;

	let db = MongoRepo::init();

	let data = Battlemech {
		id: None,
		name: battlemech.name.to_owned(),
		designation: battlemech.designation.to_owned(),
		components: battlemech.components.clone(),
	};

	for(_i, c) in data.components.iter().enumerate() {
		match Component::validate(&c.name) {
			true => continue,
			// false => panic!("Component was invalid: {}", c.name)
			false => return Err(String::from(format!("Component was invalid: {}", c.name)))
		}
	}

	let battlemech_detail = db.create_battlemech(data);
	match battlemech_detail {
		Ok(battlemech) => {
			// let response = Response { 
			// 	body: ResponseBody::Message(format!("Battlemech created: {}", battlemech.inserted_id.to_string())),
			// };
			// Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
			Ok(battlemech)
		},
		Err(err) => Err(err.to_string())
	}
}