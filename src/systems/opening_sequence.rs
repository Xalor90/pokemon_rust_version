use bevy::prelude::*;

use crate::components::opening_sequence::*;
use crate::components::transitions::FadeTransition;
use crate::events::transitions::FadeCompletedEvent;
use crate::resources::fonts::GameFonts;
use crate::resources::opening_sequence_assets::OpeningSequenceSpriteSheet;
use crate::resources::settings::WindowSettings;
use crate::resources::states::GameState;

/// Copyright animation sequence
pub fn copyright_screen_system(
	mut commands: Commands,
	game_fonts: Res<GameFonts>,
) {
	// Spawn copyright text with fade transition
	commands.spawn((
		CopyrightTextBundle::new(&game_fonts),
		CopyrightFade
	));
}

/// Load opening sequence sprite sheet
pub fn load_opening_sequence_sprites_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	// Load the texture for the opening sequence
	let texture: Handle<Image> = asset_server.load("sprites/opening_sequence.png");

	// Define the grid layout
	let frame_width: u32 = 32;
	let frame_height: u32 = 64;
	let columns: u32 = 2;
	let rows: u32 = 5;

	// Create the texture atlas
	let layout = TextureAtlasLayout::from_grid(
		UVec2::new(frame_width, frame_height),
		columns,
		rows,
		None,	// No padding
		None,	// No offset
	);

	let layout_handle = layouts.add(layout);

	commands.insert_resource(OpeningSequenceSpriteSheet {
		texture: texture,
		layout: layout_handle
	});
}

/// Opening sequence
pub fn opening_sequence_system(
	mut commands: Commands,
	window_settings: Res<WindowSettings>,
	sprite_sheet: Res<OpeningSequenceSpriteSheet>,
) {
	// Render background
	commands.spawn(OpeningBackgroundBundle::new(&window_settings));

	// Render opening sequence sprites
	commands.spawn(Sprite::from_atlas_image(
		sprite_sheet.texture.clone(),
		TextureAtlas {
			layout: sprite_sheet.layout.clone(),
			index: 8,
		},
	));
}

/// Sync the background color to the sprite color
pub fn sync_background_color_to_sprite_system(
	mut query: Query<(&BackgroundColor, &mut Sprite)>
) {
	for (background_color, mut sprite) in &mut query {
		sprite.color = background_color.0;
	}
}

/// System to handle fade completed events
pub fn handle_copyright_fade_completed_event_system(
	mut commands: Commands,
	mut events: EventReader<FadeCompletedEvent>,
	mut next_state: ResMut<NextState<GameState>>,
	copyright_query: Query<&CopyrightFade>,
) {
	for event in events.read() {
		// Despawn the entity
		commands.entity(event.entity).despawn();

		// Check if the entity is a copyright fade entity
		if copyright_query.get(event.entity).is_ok() {
			// Transition to the next state after fade is complete
			next_state.set(GameState::OpeningSequence);
		}
	}
}

/// System to handle fade completed events
pub fn handle_background_fade_completed_event_system(
	mut commands: Commands,
	mut events: EventReader<FadeCompletedEvent>,
	background_query: Query<&OpeningBackgroundFade>,
) {
	for event in events.read() {
		// Check if the entity is a background fade entity
		if background_query.get(event.entity).is_ok() {
			// Remove the fade transition component once the fade is complete
			commands.entity(event.entity).remove::<FadeTransition>();
		}
	}
}
