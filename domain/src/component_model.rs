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

impl Component {
	pub fn new(name: String, internal_structure: u32, armor: u32) -> Result<Component, &'static str> {
		if Component::validate(&name) {
			Ok(Component {
				name: name,
				internal_structure: internal_structure,
				armor: armor
			})
		}
		else {
			Err("Invalid component name")
		}
	}

	pub fn validate(name: &String) -> bool {
		let valid_names = ["head".to_string(), "center torso".to_string(), "center torso (rear)".to_string(), "left torso".to_string(), "left torso (rear)".to_string(), "right torso".to_string(), "right torso (rear)".to_string(), "left arm".to_string(), "right arm".to_string(), "left leg".to_string(), "right leg".to_string()];
		valid_names.contains(name)
	}
}