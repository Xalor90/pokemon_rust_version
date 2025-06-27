use serde::{Serialize, Deserialize};
use crate::types::PokemonType;

/// Struct for defining the base stats of a Pokemon.
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseStats {
	pub hp: u16,
	pub attack: u16,
	pub defense: u16,
	pub special_attack: u16,
	pub special_defense: u16,
	pub speed: u16,
}

/// Struct for defining a Pokemon Species.
#[derive(Debug, Serialize, Deserialize)]
pub struct Species {
	pub name: String,
	pub type_one: PokemonType,
	pub type_two: Option<PokemonType>,
	pub base_stats: BaseStats,
}