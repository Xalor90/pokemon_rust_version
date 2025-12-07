use bevy::prelude::*;

/// An event that is fired when a fade transition is completed
#[derive(Event)]
pub struct FadeCompletedEvent {
	pub entity: Entity,
}
