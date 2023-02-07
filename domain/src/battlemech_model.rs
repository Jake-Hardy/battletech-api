use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::component_model::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct Battlemech {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub name: String,
	pub designation: String,
	pub components: Vec<Component>,
}