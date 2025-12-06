use bevy::prelude::*;
use std::time::Duration;

/// Transition Enums
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeMode {
	InHoldOut,
//	InOnly,
//	OutOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeState {
	In,
	Hold,
	Out,
}

/// Transition Components
#[derive(Component)]
pub struct FadeTransition {
	pub mode: FadeMode,
	pub current_state: FadeState,
	pub timer: Timer,
	pub duration: Duration,
	pub hold: Option<Duration>,
}

/// Transition Implementations
impl FadeTransition {
	/// Full fade cycle, including fade-in, hold, and fade-out
	pub fn full_cycle(fade_secs: f32, hold_secs: f32) -> Self {
		Self {
			mode: FadeMode::InHoldOut,
			current_state: FadeState::In,
			timer: Timer::from_seconds(fade_secs, TimerMode::Once),
			duration: Duration::from_secs_f32(fade_secs),
			hold: Some(Duration::from_secs_f32(hold_secs)),
		}
	}

	// Fade in only (stays visible)
//	pub fn fade_in_only(fade_secs: f32) -> Self {
//		Self {
//			mode: FadeMode::InOnly,
//			current_state: FadeState::In,
//			timer: Timer::from_seconds(fade_secs, TimerMode::Once),
//			duration: Duration::from_secs_f32(fade_secs),
//			hold: None,
//		}
//	}

	// Fade out only (despawn at end)
//	pub fn fade_out_only(fade_secs: f32) -> Self {
//		Self {
//			mode: FadeMode::OutOnly,
//			current_state: FadeState::Out,
//			timer: Timer::from_seconds(fade_secs, TimerMode::Once),
//			duration: Duration::from_secs_f32(fade_secs),
//			hold: None,
//		}
//	}
}

/// Traits
pub trait WithColor {
	fn color_mut(&mut self) -> &mut Color;
}

pub trait TimerPercent {
	fn percent(&self) -> f32;
}

/// Trait implementations
impl WithColor for TextColor {
	fn color_mut(&mut self) -> &mut Color {
		&mut self.0
	}
}

impl TimerPercent for Timer {
	fn percent(&self) -> f32 {
		let elapsed = self.elapsed().as_secs_f32();
		let total = self.duration().as_secs_f32();
		(elapsed / total).clamp(0.0, 1.0)
	}
}
