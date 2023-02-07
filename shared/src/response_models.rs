use domain::battlemech_model::Battlemech;

use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
	Message(String),
	Battlemech(Battlemech),
	Battlemechs(Vec<Battlemech>)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
	pub body: ResponseBody,
}