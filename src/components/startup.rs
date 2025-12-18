use bevy::prelude::*;
use bevy::render::camera::{Projection, ScalingMode};

/// Marker component for the startup camera
#[derive(Component)]
pub struct StartupCamera;

/// Bundle for the startup camera
#[derive(Bundle)]
pub struct StartupCameraBundle {
	pub marker: StartupCamera,
	pub camera: Camera,
	pub camera_2d: Camera2d,
	pub projection: Projection,
	pub transform: Transform,
}

/// Implementation for StartupCameraBundle
impl StartupCameraBundle {
	pub fn new(virtual_resolution: Vec2) -> Self {
		Self {
			marker: StartupCamera,
			camera: Camera {
				..default()
			},
			camera_2d: Camera2d,
			projection: Projection::Orthographic(OrthographicProjection {
				scaling_mode: ScalingMode::FixedVertical {
					viewport_height: virtual_resolution.y,
				},
				..OrthographicProjection::default_2d()
			}),
			transform: Transform::default(),
		}
	}
}
