use serde::{Serialize, Deserialize};

/// Enum for managing regional variants of a Pokemon species.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Region {
	Kanto,
	Johto,
	Hoenn,
	Sinnoh,
	Unova,
	Kalos,
	Alola,
	Galar,
	Paldea,
}

/// Enum for managing various formes within a pokemon species.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Forme {
	Deoxys(DeoxysForme),
	Wormadam(CloakForm),
	Rotom(RotomForm),
	Special(SpecialForme),
	Transformation(EvolutionForme),
}

/// Deoxys Formes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeoxysForme {
	Normal,
	Attack,
	Defense,
	Speed,
}

/// Burmy & Wormadam Cloak Forms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloakForm {
	Plant,
	Sandy,
	Trash,
}

/// Rotom Forms.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RotomForm {
	Normal,
	Heat,
	Wash,
	Frost,
	Fan,
	Mow,
}

/// Spceial variants based on fluctuations in time or space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpecialForme {
	Altered,
	Origin,
}

/// Temporary evoltuion types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvolutionForme {
	Mega,
	Primal,
	Gigantamax,
	Eternamax,
}