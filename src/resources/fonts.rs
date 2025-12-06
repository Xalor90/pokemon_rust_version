use bevy::prelude::*;

#[derive(Resource)]
pub struct GameFonts {
	pub pokemon_gb: Handle<Font>,
}

impl Default for GameFonts {
	fn default() -> Self {
		Self {
			pokemon_gb: Handle::default(),
		}
	}
}
