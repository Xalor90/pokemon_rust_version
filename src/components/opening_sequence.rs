use bevy::prelude::*;

use crate::components::transitions::FadeTransition;
use crate::resources::fonts::GameFonts;
use crate::resources::settings::WindowSettings;
use crate::components::transitions::WithColor;

/// Opening Sequence Constants
pub const COPYRIGHT_TEXT: &str = "\u{00A9}2025         Pokemon\n\
\u{00A9}1995-2025    Nintendo\n\
\u{00A9}1995-2025    Creatures, Inc.\n\
\u{00A9}1995-2025    Game Freak, Inc.";

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub enum OpeningSequenceSystemSet {
	Fade,
	Sync,
}

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
pub struct OpeningBackgroundBundle {
	pub sprite: Sprite,
	pub transform: Transform,
	pub color: OpeningBackgroundColor,
	pub fade: FadeTransition,
	pub marker: OpeningBackgroundFade,
}

/// Component for the copyright fade effect
#[derive(Component)]
pub struct CopyrightFade;

/// Component for the background fade effect
#[derive(Component)]
pub struct OpeningBackgroundFade;

/// Component for the background color
#[derive(Component)]
pub struct OpeningBackgroundColor(pub Color);

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

/// Implementation for OpeningBackgroundBundle
impl OpeningBackgroundBundle {
	pub fn new(window_settings: &WindowSettings) -> Self {
		let background_height = window_settings.virtual_resolution.y * 3.0 / 5.0;
		let background_width = window_settings.virtual_resolution.x;

		let base_color = Color::srgba(
			24.0/255.0,
			41.0/255.0,
			74.0/255.0,
			0.0,
		);

		Self {
			sprite: Sprite {
				color: base_color,
				custom_size: Some(Vec2::new(background_width, background_height)),
				..default()
			},
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			color: OpeningBackgroundColor(base_color),
			fade: FadeTransition::fade_in_only(2.0),
			marker: OpeningBackgroundFade,
		}
	}
}

/// Implementation of the WithColor trait for TextColor
impl WithColor for TextColor {
	fn color_mut(&mut self) -> &mut Color {
		&mut self.0
	}
}

/// Implementation of WithColor for OpeningBackgroundColor
impl WithColor for OpeningBackgroundColor {
	fn color_mut(&mut self) -> &mut Color {
		&mut self.0
	}
}
