use bevy::prelude::*;

/// Game States
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
	#[default]
	Copyright,
	OpeningScene,
//	StartMenu,
//	InGame,
}
