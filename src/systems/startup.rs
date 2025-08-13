use bevy::prelude::*;
use crate::components::startup::*;
use crate::components::transitions::FadeTransition;

/// System for handling the opening sequence for the game and calling the handler for the main menu
pub fn opening_sequence(mut commands: Commands) {
	commands.spawn(Camera2d);

	commands.spawn((
		Text2d::new(COPYRIGHT_TEXT),
		TextFont {
			font_size: 32.0,
			..default()
		},
		TextColor(Color::WHITE.with_alpha(0.0)),
		FadeTransition::full_cycle(5.0, 2.0),
	));
}
