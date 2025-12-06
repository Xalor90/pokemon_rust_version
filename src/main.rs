use bevy::prelude::*;
use crate::resources::fonts::GameFonts;
use crate::resources::states::GameState;
use crate::systems::startup::*;
use crate::systems::transitions::*;

mod components;
mod resources;
mod systems;

fn main() {
	let mut app = App::new();

	let window = WindowPlugin {
		primary_window: Some(Window {
			title: "Pokemon: Rust Version".to_string(),
			resolution: (1920.0, 1080.0).into(),
			..default()
		}),
		..default()
	};
	app.add_plugins(DefaultPlugins.set(window));
	app.insert_resource(ClearColor(Color::BLACK));

	app.init_resource::<GameFonts>();
	app.init_state::<GameState>();
	app.insert_resource(NextState::<GameState>::default());

	app.add_systems(OnEnter(GameState::Loading), setup_game_fonts_system);
	app.add_systems(Update, check_game_fonts_system.run_if(in_state(GameState::Loading)));

	app.add_systems(Startup, startup_system);
	app.add_systems(OnEnter(GameState::Copyright), copyright_screen);
	app.add_systems(OnEnter(GameState::OpeningScene), opening_scene);
	app.add_systems(Update, fade_system::<TextColor>);

	app.run();
}
