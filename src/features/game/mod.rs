mod game_selector;
mod state;
mod in_game;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((game_selector::GameSelectorPlugin, state::GameStatePlugin));
    }
}
