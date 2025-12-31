use bevy::prelude::*;

/// Assets for the opening sequence
#[derive(Resource)]
pub struct OpeningSequenceSpriteSheet {
	pub texture: Handle<Image>,
	pub layout: Handle<TextureAtlasLayout>,
}
