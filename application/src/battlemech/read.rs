use domain::battlemech_model::Battlemech;

use rocket::http::Status;

pub fn list_battlemech(path: String) -> Result<Battlemech, Status> {
	use domain::mongodb_repo::MongoRepo;

	let db = MongoRepo::init();

	let id = path;
	if id.is_empty() {
		return Err(Status::BadRequest);
	};
	let battlemech_detail = db.get_battlemech(&id);
	match battlemech_detail {
		Ok(battlemech) => Ok(battlemech),
		Err(_) => Err(Status::InternalServerError),
	}
}

pub fn list_battlemechs() -> Result<Vec<Battlemech>, Status> {
	use domain::mongodb_repo::MongoRepo;

	let db = MongoRepo::init();

	let battlemechs = db.get_all_battlemechs();
	match battlemechs {
		Ok(battlemechs) => Ok(battlemechs),
		Err(_) => Err(Status::InternalServerError),
	}
}