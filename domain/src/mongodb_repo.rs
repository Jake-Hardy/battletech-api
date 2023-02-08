use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
	bson::{extjson::de::Error, oid::ObjectId, doc},
	results::{InsertOneResult, UpdateResult, DeleteResult},
	sync::{Client, Collection},
};

use crate::{battlemech_model::Battlemech};

pub struct MongoRepo {
	col: Collection<Battlemech>,
}

impl MongoRepo {
	pub fn init() -> Self {
		dotenv().ok();
		let uri = match env::var("MONGOURI") {
			Ok(v) => v.to_string(),
			Err(_) => format!("Error loading env variable"),
		};
		let client = Client::with_uri_str(uri).unwrap();
		let db = client.database("rustDB");
		let col: Collection<Battlemech> = db.collection("Battlemech");
		MongoRepo { col }
	}

	pub fn create_battlemech(&self, new_mech: Battlemech) -> Result<Battlemech, Error> {
		let new_doc = Battlemech {
			id: None,
			name: new_mech.name,
			designation: new_mech.designation,
			components: new_mech.components,
		};
		let mech = self 
			.col 
			.insert_one(new_doc, None)
			.ok()
			.expect("Error creating mech");
		let mech_id = mech.inserted_id.into_relaxed_extjson().get("$oid").unwrap().to_string();
		
		// This is so fucking hacky
		self.get_battlemech(&mech_id[1..mech_id.len() - 1].to_string())
	}

	pub fn get_all_battlemechs(&self) -> Result<Vec<Battlemech>, Error> {
		let cursors = self 
			.col 
			.find(None, None)
			.ok()
			.expect("Error getting list of Battlemechs");
		let battlemechs = cursors.map(|doc| doc.unwrap()).collect();
		Ok(battlemechs)
	}

	pub fn get_battlemech(&self, id: &String) -> Result<Battlemech, Error> {
		let obj_id = match ObjectId::parse_str(id) {
			Ok(i) => i,
			Err(e) => return Err(mongodb::bson::extjson::de::Error::InvalidObjectId(e)),
		};
		let filter = doc! {"_id": obj_id};
		let battlemech_detail = self 
			.col 
			.find_one(filter, None)
			.ok()
			.expect("Error getting Battlemech's detail");
		Ok(battlemech_detail.unwrap())
	}

	pub fn update_battlemech(&self, id: &String, new_battlemech: Battlemech) -> Result<UpdateResult, Error> {
		let obj_id = ObjectId::parse_str(id).unwrap();
		let filter = doc! {"_id": obj_id};
		let new_doc = doc! {
			"$set":
				{
					"id": new_battlemech.id,
					"name": new_battlemech.name,
					"designation": new_battlemech.designation,
					"components": new_battlemech.components,
				},
		};
		let updated_doc = self 
			.col 
			.update_one(filter, new_doc, None)
			.ok()
			.expect("Error updating Battlemech");
		Ok(updated_doc)
	}

	pub fn delete_battlemech(&self, id: &String) -> Result<DeleteResult, Error> {
		let obj_id = ObjectId::parse_str(id).unwrap();
		let filter = doc! {"_id": obj_id};
		let battlemech_detail = self 
			.col 
			.delete_one(filter, None)
			.ok()
			.expect("Error deleting Battlemech");
		Ok(battlemech_detail)
	}
}