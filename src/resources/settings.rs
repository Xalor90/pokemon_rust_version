use bevy::prelude::*;
use bevy::window::{MonitorSelection, PresentMode, WindowMode};

/// Resource for window settings
#[derive(Resource)]
pub struct WindowSettings {
	pub virtual_resolution: Vec2,
	pub window_title: String,
	pub window_mode: WindowMode,
	pub present_mode: PresentMode,
}

/// Default implementation for WindowSettings
impl Default for WindowSettings {
	fn default() -> Self {
		Self {
			virtual_resolution: Vec2::new(3840.0, 2160.0),
			window_title: "Pokemon Rust Version".to_string(),
			window_mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
			present_mode: PresentMode::AutoVsync,
		}
	}
}
