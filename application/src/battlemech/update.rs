use domain::battlemech_model::Battlemech;
use mongodb::bson::oid::ObjectId;

use rocket::{http::Status, serde::json::Json};

pub fn update_battlemech(battlemech_id: String, new_battlemech: Json<Battlemech>) -> Result<Battlemech, Status>{
	use domain::mongodb_repo::MongoRepo;

	let db = MongoRepo::init();

	if battlemech_id.is_empty() {
		return Err(Status::BadRequest);
	};
	let data = Battlemech {
		id: Some(ObjectId::parse_str(&battlemech_id).unwrap()),
		name: new_battlemech.name.to_owned(),
		designation: new_battlemech.designation.to_owned(),
		components: new_battlemech.components.clone(),
	};
	let update_result = db.update_battlemech(&battlemech_id, data);
	match update_result {
		Ok(update) => {
			if update.matched_count == 1 {
				let updated_battlemech_info = db.get_battlemech(&battlemech_id);
				return match updated_battlemech_info {
					Ok(battlemech) => Ok(battlemech),
					Err(_) => Err(Status::InternalServerError),
				};
			} else {
				return Err(Status::NotFound);
			}
		}
		Err(_) => Err(Status::InternalServerError),
	}
}