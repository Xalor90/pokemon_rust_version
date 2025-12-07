use bevy::prelude::*;

/// Marker component for the startup camera
#[derive(Component)]
pub struct StartupCamera;

/// Bundle for the initial camera used at startup
#[derive(Bundle)]
pub struct InitialCameraBundle {
	pub marker: StartupCamera,
	pub camera: Camera2d,
	pub camera_transform: Transform,
}

/// Default implementation for InitialCameraBundle
impl Default for InitialCameraBundle {
	fn default() -> Self {
		Self {
			marker: StartupCamera,
			camera: Camera2d {
				..default()
			},
			camera_transform: Transform {
				..default()
			},
		}
	}
}
