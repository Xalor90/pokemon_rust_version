/// An enum for all Pokemon Types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PokemonType {
	Normal,
	Fire,
	Water,
	Electric,
	Grass,
	Ice,
	Fighting,
	Poison,
	Ground,
	Flying,
	Psychic,
	Bug,
	Rock,
	Ghost,
	Dragon,
	Dark,
	Steel,
	Fairy,
}

/// An enum for DamageModifiers based on how effective a move is against a Pokemon type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageModifier {
	NoEffect,
	NotVeryEffective,
	Effective,
	SuperEffective,
}

impl DamageModifier {
	/// Returns the multiplier for the damage modifier.
	pub fn multiplier(self) -> f32 {
		match self {
			DamageModifier::NoEffect => 0.0,
			DamageModifier::NotVeryEffective => 0.5,
			DamageModifier::Effective => 1.0,
			DamageModifier::SuperEffective => 2.0,
		}
	}
}

/// Returns the damage modifier for a given attacking and defending Pokemon type.
pub fn type_matchup(attacking: PokemonType, defending: PokemonType) -> DamageModifier {
	use PokemonType::*;
	use DamageModifier::*;

	match (attacking, defending) {
		// Normal type matchups
		(Normal, Rock)			=> NotVeryEffective,
		(Normal, Ghost)			=> NoEffect,
		(Normal, Steel)			=> NotVeryEffective,

		// Fire type matchups
		(Fire, Fire)			=> NotVeryEffective,
		(Fire, Water)			=> NotVeryEffective,
		(Fire, Grass)			=> SuperEffective,
		(Fire, Ice)				=> SuperEffective,
		(Fire, Bug)				=> SuperEffective,
		(Fire, Rock)			=> NotVeryEffective,
		(Fire, Dragon)			=> NotVeryEffective,
		(Fire, Steel)			=> SuperEffective,

		// Water type matchups
		(Water, Fire)			=> SuperEffective,
		(Water, Water)			=> NotVeryEffective,
		(Water, Grass)			=> NotVeryEffective,
		(Water, Ground)			=> SuperEffective,
		(Water, Rock)			=> SuperEffective,
		(Water, Dragon)			=> NotVeryEffective,

		// Electric type matchups
		(Electric, Water)		=> SuperEffective,
		(Electric, Electric)	=> NotVeryEffective,
		(Electric, Grass)		=> NotVeryEffective,
		(Electric, Ground)		=> NoEffect,
		(Electric, Flying)		=> SuperEffective,
		(Electric, Dragon)		=> NotVeryEffective,

		// Grass type matchups
		(Grass, Fire)			=> NotVeryEffective,
		(Grass, Water)			=> SuperEffective,
		(Grass, Grass)			=> NotVeryEffective,
		(Grass, Poison)			=> NotVeryEffective,
		(Grass, Ground)			=> SuperEffective,
		(Grass, Flying)			=> NotVeryEffective,
		(Grass, Bug)			=> NotVeryEffective,
		(Grass, Rock)			=> SuperEffective,
		(Grass, Dragon)			=> NotVeryEffective,
		(Grass, Steel)			=> NotVeryEffective,
		
		// Ice type matchups
		(Ice, Fire)				=> NotVeryEffective,
		(Ice, Water)			=> NotVeryEffective,
		(Ice, Grass)			=> SuperEffective,
		(Ice, Ice)				=> NotVeryEffective,
		(Ice, Ground)			=> SuperEffective,
		(Ice, Flying)			=> SuperEffective,
		(Ice, Dragon)			=> SuperEffective,
		(Ice, Steel)			=> NotVeryEffective,

		// Fighting type matchups
		(Fighting, Normal)		=> SuperEffective,
		(Fighting, Ice)			=> SuperEffective,
		(Fighting, Poison)		=> NotVeryEffective,
		(Fighting, Flying)		=> NotVeryEffective,
		(Fighting, Psychic)		=> NotVeryEffective,
		(Fighting, Bug)			=> NotVeryEffective,
		(Fighting, Rock)		=> SuperEffective,
		(Fighting, Ghost)		=> NoEffect,
		(Fighting, Dark)		=> SuperEffective,
		(Fighting, Steel)		=> SuperEffective,
		(Fighting, Fairy)		=> NotVeryEffective,
		
		// Poison type matchups
		(Poison, Grass)			=> SuperEffective,
		(Poison, Poison)		=> NotVeryEffective,
		(Poison, Ground)		=> NotVeryEffective,
		(Poison, Rock)			=> NotVeryEffective,
		(Poison, Ghost)			=> NotVeryEffective,
		(Poison, Steel)			=> NoEffect,
		(Poison, Fairy)			=> SuperEffective,

		// Ground type matchups
		(Ground, Fire)			=> SuperEffective,
		(Ground, Electric)		=> SuperEffective,
		(Ground, Grass)			=> NotVeryEffective,
		(Ground, Poison)		=> SuperEffective,
		(Ground, Flying)		=> NoEffect,
		(Ground, Bug)			=> NotVeryEffective,
		(Ground, Rock)			=> SuperEffective,
		(Ground, Steel)			=> SuperEffective,
		
		// Flying type matchups
		(Flying, Electric)		=> NotVeryEffective,
		(Flying, Grass)			=> SuperEffective,
		(Flying, Fighting)		=> SuperEffective,
		(Flying, Bug)			=> SuperEffective,
		(Flying, Rock)			=> NotVeryEffective,
		(Flying, Steel)			=> NotVeryEffective,

		// Psychic type matchups
		(Psychic, Fighting)		=> SuperEffective,
		(Psychic, Poison)		=> SuperEffective,
		(Psychic, Psychic)		=> NotVeryEffective,
		(Psychic, Dark)			=> NoEffect,
		(Psychic, Steel)		=> NotVeryEffective,
		
		// Bug type matchups
		(Bug, Fire)				=> NotVeryEffective,
		(Bug, Grass)			=> SuperEffective,
		(Bug, Fighting)			=> NotVeryEffective,
		(Bug, Poison)			=> NotVeryEffective,
		(Bug, Flying)			=> NotVeryEffective,
		(Bug, Psychic)			=> SuperEffective,
		(Bug, Ghost)			=> NotVeryEffective,
		(Bug, Dark)				=> SuperEffective,
		(Bug, Steel)			=> NotVeryEffective,
		(Bug, Fairy)			=> NotVeryEffective,

		// Rock type matchups
		(Rock, Fire)			=> SuperEffective,
		(Rock, Ice)				=> SuperEffective,
		(Rock, Fighting)		=> NotVeryEffective,
		(Rock, Ground)			=> NotVeryEffective,
		(Rock, Flying)			=> SuperEffective,
		(Rock, Bug)				=> SuperEffective,
		(Rock, Steel)			=> NotVeryEffective,

		// Ghost type matchups
		(Ghost, Normal)			=> NoEffect,
		(Ghost, Psychic)		=> SuperEffective,
		(Ghost, Ghost)			=> SuperEffective,
		(Ghost, Dark)			=> NotVeryEffective,

		// Dragon type matchups
		(Dragon, Dragon)		=> SuperEffective,
		(Dragon, Steel)			=> NotVeryEffective,
		(Dragon, Fairy)			=> NoEffect,

		// Dark type matchups
		(Dark, Fighting)		=> NotVeryEffective,
		(Dark, Psychic)			=> SuperEffective,
		(Dark, Ghost)			=> SuperEffective,
		(Dark, Dark)			=> NotVeryEffective,
		(Dark, Fairy)			=> NotVeryEffective,

		// Steel type matchups
		(Steel, Fire)			=> NotVeryEffective,
		(Steel, Water)			=> NotVeryEffective,
		(Steel, Electric)		=> NotVeryEffective,
		(Steel, Ice)			=> SuperEffective,
		(Steel, Rock)			=> SuperEffective,
		(Steel, Steel)			=> NotVeryEffective,
		(Steel, Fairy)			=> SuperEffective,

		// Fairy type matchups
		(Fairy, Fire)			=> NotVeryEffective,
		(Fairy, Fighting)		=> SuperEffective,
		(Fairy, Poison)			=> NotVeryEffective,
		(Fairy, Dragon)			=> SuperEffective,
		(Fairy, Dark)			=> SuperEffective,
		(Fairy, Steel)			=> NotVeryEffective,

		// Default to Effective
		_ => Effective
	}
}