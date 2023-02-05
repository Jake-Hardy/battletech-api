use serde::{Serialize, Deserialize};
use mongodb::bson::{self, Bson, doc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Component {
	pub name: String,
	pub internal_structure: u32,
	pub armor: u32,
}

impl Into<Bson> for Component {
	fn into(self) -> bson::Bson {
		bson::Bson::Document(doc! {
			"name": self.name,
			"internal_structure": self.internal_structure,
			"armor": self.armor
		})
	}
}