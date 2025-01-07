mod setup;
mod state;

use bevy::prelude::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((state::InGameStatePlugin, setup::InGameSetupPlugin));
    }
}
