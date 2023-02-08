use shared::response_models::{Response, ResponseBody};
use application::battlemech::{create, read, update, delete};
use domain::battlemech_model::Battlemech;
use rocket::{get, post, put, delete};
use rocket::response::content;
use rocket::response::status::Created;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/battlemech")]
pub fn list_battlemechs_handler() -> content::RawJson<String> {
	let battlemechs: Vec<Battlemech> = read::list_battlemechs().unwrap();
	let response = Response { body: ResponseBody::Battlemechs(battlemechs)};

	content::RawJson(serde_json::to_string(&response).unwrap())
}

#[get("/battlemech/<battlemech_id>")]
pub fn list_battlemech_handler(battlemech_id: String) -> Result<content::RawJson<String>, Status> {
	// let battlemech = read::list_battlemech(battlemech_id)?;
	// let response = Response { body: ResponseBody::Battlemech(battlemech)};
	let response = match read::list_battlemech(battlemech_id) {
		Ok(b) => {
			Response { body: ResponseBody::Battlemech(b)}
		},
		Err(e) => {
			Response { body: ResponseBody::Message(e.to_string()) }
		}
	};

	Ok(content::RawJson(serde_json::to_string(&response).unwrap()))
}

#[post("/battlemech", format = "application/json", data = "<battlemech>")]
pub fn create_battlemech_handler(battlemech: Json<Battlemech>) -> Result<content::RawJson<String>, Status> {
	let response = match create::create_battlemech(battlemech) {
		Ok(b) => {
			Response { body: ResponseBody::Battlemech(b) }
		},
		Err(e) => {
			Response { body: ResponseBody::Message(e.to_string()) }
		}
	};

	Ok(content::RawJson(serde_json::to_string(&response).unwrap()))
}

#[put("/battlemech/<battlemech_id>", data="<new_battlemech>")]
pub fn update_battlemech_handler(battlemech_id: String, new_battlemech: Json<Battlemech>) -> Result<content::RawJson<String>, Status> {
	let battlemech = update::update_battlemech(battlemech_id, new_battlemech)?;
	let response = Response { body: ResponseBody::Battlemech(battlemech) };

	Ok(content::RawJson(serde_json::to_string(&response).unwrap()))
}

#[delete("/battlemech/<battlemech_id>")]
pub fn delete_battlemech_handler(battlemech_id: String) -> Result<content::RawJson<String>, Status> {
	let battlemech = delete::delete_battlemech(battlemech_id)?;
	let response = Response { body: ResponseBody::Message(battlemech)};

	Ok(content::RawJson(serde_json::to_string(&response).unwrap()))
}