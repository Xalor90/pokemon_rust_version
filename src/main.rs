use bevy::prelude::*;

use crate::events::transitions::FadeCompletedEvent;
use crate::resources::settings::WindowSettings;
use crate::resources::fonts::GameFonts;
use crate::resources::states::GameState;
use crate::systems::opening_sequence::*;
use crate::systems::startup::*;
use crate::systems::transitions::*;

mod components;
mod events;
mod resources;
mod systems;

/// Main function to run the Bevy app
fn main() {
	// Create Bevy app
	let mut app = App::new();
	
	// Initialize configurations
	app.init_resource::<WindowSettings>();

	// Configure window
	let window = WindowPlugin {
		primary_window: Some(Window {
			title: WindowSettings::default().window_title,
			resolution: WindowSettings::default().window_size.into(),
			..default()
		}),
		..default()
	};
	app.add_plugins(DefaultPlugins.set(window));
	app.insert_resource(ClearColor(Color::BLACK));

	// Initialize resources and states
	app.init_resource::<GameFonts>();
	app.init_state::<GameState>();
	app.insert_resource(NextState::<GameState>::default());

	// Register Events
	app.add_event::<FadeCompletedEvent>();

	// Add Update Systems
	app.add_systems(Update, check_game_fonts_system.run_if(in_state(GameState::Startup)));
	app.add_systems(Update, fade_system::<TextColor>);
	app.add_systems(Update, handle_fade_completed_event_system);

	// Add Startup Systems
	app.add_systems(OnEnter(GameState::Startup), setup_game_fonts_system);
	app.add_systems(Startup, startup_system);

	// Add Opening Sequence Systems
	app.add_systems(OnEnter(GameState::CopyrightScreen), copyright_screen_system);
	app.add_systems(OnEnter(GameState::OpeningSequence), opening_sequence_system);

	// Run the app
	app.run();
}
