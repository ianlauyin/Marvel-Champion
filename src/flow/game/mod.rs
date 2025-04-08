mod pre_game;
mod resource;
mod state;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<state::GameState>()
            .add_plugins((resource::GameResourcePlugin,));
    }
}
