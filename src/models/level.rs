use ron::de::{from_str};
use serde::{Serialize, Deserialize};
use crate::tile::TileState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Level{
	pub name: String,
	pub number: u8,
	pub values: Vec<u8>,
	pub states: Vec<bool>,
	pub flagged: Vec<TileState>,
	pub moves: u8,
}

pub fn load_level(path: &str) -> Level{
	match from_str(path){
		Ok(level) => level,
		Err(error) => {
			println!("Failed to load level: {}", error);
			std::process::exit(1);
		}
	}
}