use bevy::prelude::*;

/// Resource holding game font handles
#[derive(Resource)]
pub struct GameFonts {
	pub pokemon_gb: Handle<Font>,
}

/// Default implementation for GameFonts
impl Default for GameFonts {
	fn default() -> Self {
		Self {
			pokemon_gb: Handle::default(),
		}
	}
}
