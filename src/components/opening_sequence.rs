use bevy::prelude::*;
use crate::components::transitions::FadeTransition;
use crate::resources::fonts::GameFonts;
use crate::resources::settings::WindowSettings;

/// Opening Sequence Constants
pub const COPYRIGHT_TEXT: &str = "\u{00A9}2025         Pokemon\n\
\u{00A9}1995-2025    Nintendo\n\
\u{00A9}1995-2025    Creatures, Inc.\n\
\u{00A9}1995-2025    Game Freak, Inc.";

/// Copyright Text Bundle
#[derive(Bundle)]
pub struct CopyrightTextBundle {
	pub text: Text2d,
	pub font: TextFont,
	pub color: TextColor,
	pub fade: FadeTransition,
}

/// Background Bundle
#[derive(Bundle)]
pub struct BackgroundBundle {
	pub sprite: Sprite,
	pub transform: Transform,
	pub fade: FadeTransition,
}

/// Component for the copyright fade effect
#[derive(Component)]
pub struct CopyrightFade;

/// Implementation for CopyrightTextBundle
impl CopyrightTextBundle {
	pub fn new(game_fonts: &GameFonts) -> Self {
		Self {
			text: Text2d::new(COPYRIGHT_TEXT),
			font: TextFont {
				font: game_fonts.pokemon_gb.clone(),
				font_size: 32.0,
				..default()
			},
			color: TextColor(Color::WHITE.with_alpha(0.0)),
			fade: FadeTransition::full_cycle(5.0, 2.0),
		}
	}
}

/// Implementation for BackgroundBundle
impl BackgroundBundle {
	pub fn new(window_settings: &WindowSettings) -> Self {
		let background_height = window_settings.virtual_resolution.y * 3.0 / 5.0;
		let background_width = window_settings.virtual_resolution.x;

		Self {
			sprite: Sprite {
				color: Color::srgb(24.0/255.0, 41.0/255.0, 74.0/255.0),
				custom_size: Some(Vec2::new(background_width, background_height)),
				..default()
			},
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			fade: FadeTransition::fade_in_only(5.0),
		}
	}
}
