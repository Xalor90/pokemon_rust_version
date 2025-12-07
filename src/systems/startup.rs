use bevy::prelude::*;
use crate::components::startup::InitialCameraBundle;
use crate::resources::fonts::GameFonts;
use crate::resources::states::GameState;

/// Startup system for setting up the game fonts
pub fn setup_game_fonts_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let game_fonts = GameFonts {
		pokemon_gb: asset_server.load("fonts/pokemon_gb.ttf"),
	};
	
	commands.insert_resource(game_fonts);
}

/// Check fonts to ensure they loaded properly
pub fn check_game_fonts_system(
	asset_server: Res<AssetServer>,
	game_fonts: Res<GameFonts>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	if asset_server.is_loaded(&game_fonts.pokemon_gb) {
		next_state.set(GameState::CopyrightScreen);
	}
}

/// Startup system
pub fn startup_system(mut commands: Commands) {
	commands.spawn(InitialCameraBundle::default());
}
