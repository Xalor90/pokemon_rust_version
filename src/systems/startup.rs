use bevy::prelude::*;
use crate::components::startup::*;
use crate::components::transitions::FadeTransition;
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
		println!("Pokemon GB font is loaded!");
		next_state.set(GameState::Copyright);
	}
}

/// Startup system
pub fn startup_system(mut commands: Commands) {
	commands.spawn(Camera2d);
}

/// Copyright screen
pub fn copyright_screen(
	mut commands: Commands,
//	mut next_state: ResMut<NextState<GameState>>,
	asset_server: Res<AssetServer>,
	game_fonts: Res<GameFonts>,
) {
	if asset_server.is_loaded(&game_fonts.pokemon_gb) {
		println!("Pokemon GB font is loaded!");
	}

	commands.spawn((
		Text2d::new(COPYRIGHT_TEXT),
		TextFont {
			font: game_fonts.pokemon_gb.clone(),
			font_size: 32.0,
			..default()
		},
		TextColor(Color::WHITE.with_alpha(0.0)),
		FadeTransition::full_cycle(5.0, 2.0),
	));
}

/// Opening scene, short animated cutscene before start menu
pub fn opening_scene(mut commands: Commands) {
	commands.spawn((
		Text2d::new("Opening Scene with play now..."),
		TextFont {
			font_size: 24.0,
			..default()
		},
		TextColor(Color::WHITE),
	));
}
