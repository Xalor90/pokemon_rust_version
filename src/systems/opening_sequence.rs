use bevy::prelude::*;
use crate::components::opening_sequence::*;
use crate::resources::fonts::GameFonts;
use crate::resources::settings::WindowSettings;

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

/// Opening sequence
pub fn opening_sequence_system(
	mut commands: Commands,
	window_settings: Res<WindowSettings>,
) {
	// Render background
	commands.spawn(BackgroundBundle::new(&window_settings));
}
