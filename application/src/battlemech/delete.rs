use rocket::http::Status;

pub fn delete_battlemech(battlemech_id: String) -> Result<String, Status> {
	use domain::mongodb_repo::MongoRepo;

	let db = MongoRepo::init();

	if battlemech_id.is_empty() {
		return Err(Status::BadRequest);
	};
	let result = db.delete_battlemech(&battlemech_id);
	match result {
		Ok(res) => {
			if res.deleted_count == 1 {
				return Ok(String::from("Battlemech successfully deleted!"));
			} else {
				return Err(Status::NotFound);
			}
		}
		Err(_) => Err(Status::InternalServerError),
	}
}