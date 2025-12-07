use bevy::prelude::*;

/// Resource for window settings
#[derive(Resource)]
pub struct WindowSettings {
	pub window_title: String,
	pub window_size: Vec2,
}

/// Default implementation for WindowSettings
impl Default for WindowSettings {
	fn default() -> Self {
		Self {
			window_title: "Pokemon Rust Version".to_string(),
			window_size: Vec2::new(1280.0, 720.0),
		}
	}
}
