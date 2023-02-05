use crate::{models::battlemech_model::Battlemech, repository::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult, bson::oid::ObjectId, bson};
use rocket::{http::Status, serde::json::Json, State};

#[post("/battlemech", data = "<new_battlemech>")]
pub fn create_battlemech(
	db: &State<MongoRepo>,
	new_battlemech: Json<Battlemech>,
) -> Result<Json<InsertOneResult>, Status> {
	let data = Battlemech {
		id: None,
		name: new_battlemech.name.to_owned(),
		designation: new_battlemech.designation.to_owned(),
		components: new_battlemech.components.clone(),
	};
	let battlemech_detail = db.create_battlemech(data);
	match battlemech_detail {
		Ok(battlemech) => Ok(Json(battlemech)),
		Err(_) => Err(Status::InternalServerError),
	}
}

#[get("/battlemech")]
pub fn get_all_battlemechs(db: &State<MongoRepo>) -> Result<Json<Vec<Battlemech>>, Status> {
	let battlemechs = db.get_all_battlemechs();
	match battlemechs {
		Ok(battlemechs) => Ok(Json(battlemechs)),
		Err(_) => Err(Status::InternalServerError),
	}
}

#[get("/battlemech/<path>")]
pub fn get_battlemech(db: &State<MongoRepo>, path: String) -> Result<Json<Battlemech>, Status> {
	let id = path;
	if id.is_empty() {
		return Err(Status::BadRequest);
	};
	let battlemech_detail = db.get_battlemech(&id);
	match battlemech_detail {
		Ok(battlemech) => Ok(Json(battlemech)),
		Err(_) => Err(Status::InternalServerError),
	}
}

#[put("/battlemech/<path>", data = "<new_battlemech>")]
pub fn update_battlemech(
	db: &State<MongoRepo>,
	path: String,
	new_battlemech: Json<Battlemech>,
) -> Result<Json<Battlemech>, Status> {
	let id = path;
	if id.is_empty() {
		return Err(Status::BadRequest);
	};
	let data = Battlemech {
		id: Some(ObjectId::parse_str(&id).unwrap()),
		name: new_battlemech.name.to_owned(),
		designation: new_battlemech.designation.to_owned(),
		components: new_battlemech.components.clone(),
	};
	let update_result = db.update_battlemech(&id, data);
	match update_result {
		Ok(update) => {
			if update.matched_count == 1 {
				let updated_battlemech_info = db.get_battlemech(&id);
				return match updated_battlemech_info {
					Ok(battlemech) => Ok(Json(battlemech)),
					Err(_) => Err(Status::InternalServerError),
				};
			} else {
				return Err(Status::NotFound);
			}
		}
		Err(_) => Err(Status::InternalServerError),
	}
}

#[delete("/battlemech/<path>")]
pub fn delete_battlemech(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
	let id = path;
	if id.is_empty() {
		return Err(Status::BadRequest);
	};
	let result = db.delete_battlemech(&id);
	match result {
		Ok(res) => {
			if res.deleted_count == 1 {
				return Ok(Json("Battlemech successfully deleted!"));
			} else {
				return Err(Status::NotFound);
			}
		}
		Err(_) => Err(Status::InternalServerError),
	}
}