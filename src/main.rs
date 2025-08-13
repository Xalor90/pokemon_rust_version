use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use crate::systems::startup::*;
use crate::systems::transitions::*;

mod components;
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

	load_internal_binary_asset!(
		app,
		TextFont::default().font,
		"../assets/fonts/pokemon_gb.ttf",
		|bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
	);

	app.add_systems(Startup, opening_sequence);
	app.add_systems(Update, fade_system::<TextColor>);

	app.run();
}
