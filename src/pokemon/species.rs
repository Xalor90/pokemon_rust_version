use std::fmt::{self, Display};
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::types::PokemonType;
use crate::pokemon::variants::{Region, Forme};

/// Struct for defining the base stats of a Pokemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseStats {
	pub hp: u16,
	pub attack: u16,
	pub defense: u16,
	pub special_attack: u16,
	pub special_defense: u16,
	pub speed: u16,
}

/// Struct for handling variants within species of Pokemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
	pub is_default: bool,
	pub region: Option<Region>,
	pub forme: Option<Forme>,
	pub type_one: Option<PokemonType>,
	pub type_two: Option<PokemonType>,
	pub base_stats: Option<BaseStats>,
}

/// Struct for defining a Pokemon Species.
#[derive(Debug, Serialize, Deserialize)]
pub struct Species {
	pub name: String,
	pub selected_variant: Option<Variant>,
	pub variants: Vec<Variant>,
}

impl Display for Variant {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Region: {:?}, Forme: {:?}, Type 1: {:?}, Type 2: {:?}, Stats: {:?}",
			self.region, self.forme, self.type_one, self.type_two, self.base_stats
		)
	}
}

impl Variant {
	/// Validate the default Variant has all required fields.
	fn validate_default(&self) -> Result<(), String> {
		if self.is_default {
			if self.region.is_none() {
				return Err("Default variant missing region".into());
			}
			if self.type_one.is_none() {
				return Err("Default variant missing type_one".into());
			}
			if self.base_stats.is_none() {
				return Err("Default variant missing base_stats".into());
			}
		}
		Ok(())
	}
}

impl Species {
	pub fn from_json_file(path: &str, region: Option<Region>, forme: Option<Forme>) -> Result<Self, String> {
		let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
		let mut species: Species = serde_json::from_str(&data).map_err(|e| e.to_string())?;
		species.validate()?;
		let selected = species.resolved_variant(region, forme).ok_or_else(|| "Requested variant not found".to_string())?;
		species.selected_variant = Some(selected);
		Ok(species)
	}
	
	/// Validate the Species has exactly ONE default variant.
	fn validate(&self) -> Result<(), String> {
		let defaults: Vec<_> = self.variants.iter().filter(|v| v.is_default).collect();
		if defaults.len() != 1 {
			return Err("Species must have exactly one default variant".into());
		}
		defaults[0].validate_default()?;
		Ok(())
	}

	/// Resolve variant by ensuring all fields have values or use the values from the default variant.
	fn resolved_variant(&self, region: Option<Region>, forme: Option<Forme>) -> Option<Variant> {
		let default = self.default_variant()?.clone();
		let variant = self.variants.iter().find(|v| {
			(region.is_none() || v.region == region)
				&& (forme.is_none() || v.forme == forme)
				&& !v.is_default
		});

		if let Some(override_v) = variant {
			Some(merge_variants(&default, override_v))
		} else {
			Some(default.clone())
		}
	}

	/// Get the default variant
	fn default_variant(&self) -> Option<&Variant> {
		self.variants.iter().find(|v| v.is_default)
	}
}

/// Merge two variants, using override's fields if present, otherwise default's.
fn merge_variants(default: &Variant, override_v: &Variant) -> Variant {
	Variant {
		is_default: false,
		region: override_v.region.clone().or(default.region.clone()),
		forme: override_v.forme.clone().or(default.forme.clone()),
		type_one: override_v.type_one.or(default.type_one),
		type_two: override_v.type_two.or(default.type_two),
		base_stats: override_v.base_stats.clone().or(default.base_stats.clone()),
	}
}