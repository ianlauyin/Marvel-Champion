mod game_selector;
mod in_game;
mod state;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_selector::GameSelectorPlugin,
            in_game::InGamePlugin,
            state::GameStatePlugin,
        ));
    }
}
