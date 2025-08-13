use bevy::{ecs::component::Mutable, prelude::*};
use crate::components::transitions::{FadeTransition, FadeMode, FadeState, WithColor, TimerPercent};

/// System for handling fade transitions between screens
pub fn fade_system<T: Component + WithColor>(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut T, &mut FadeTransition)>,
) where T: Component<Mutability = Mutable> + WithColor {
	for (entity, mut color_holder, mut fade) in query.iter_mut() {
		fade.timer.tick(time.delta());

		// Set alpha based on FadeMode & FadeState by using the percent
		let percent = fade.timer.percent();
		let new_alpha = match fade.current_state {
			FadeState::In => percent,
			FadeState::Hold => 1.0,
			FadeState::Out => 1.0 - percent,
		};
		color_holder.color_mut().set_alpha(new_alpha);

		if !fade.timer.finished() {
			continue;
		}

		fade.timer.reset();
		match fade.mode {
			FadeMode::InHoldOut => {
				fade.current_state = match fade.current_state {
					FadeState::In => {
						let hold_duration = fade.hold.expect("Always set if `FadeMode::InHoldOut`.");
						fade.timer.set_duration(hold_duration);
						FadeState::Hold
					}
					FadeState::Hold => {
						let fade_duration = fade.duration;
						fade.timer.set_duration(fade_duration);
						FadeState::Out
					}
					FadeState::Out => {
						commands.entity(entity).despawn();
						continue;
					}
				}
			}

			FadeMode::InOnly => {
				continue;
			}

			FadeMode::OutOnly => {
				commands.entity(entity).despawn();
				continue;
			}
		}
	}
}
