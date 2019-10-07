use ron::de::{from_str};
use serde::{Serialize, Deserialize};
use crate::tile::TileState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Level{
	pub name: String,
	pub number: u8,
	pub values: Vec<u8>,
	#[serde(default = "default_states")]
	pub states: Vec<bool>,
	#[serde(default = "default_flagged")]
	pub flagged: Vec<TileState>,
	pub moves: u8,
	#[serde(default)]
	pub remaining_moves: u8,
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

// default values
fn default_flagged() -> Vec<TileState>{
	vec![TileState::Normal; 9]
}

fn default_states() -> Vec<bool>{
	vec![true; 9]
}