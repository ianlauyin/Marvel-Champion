use bevy::prelude::*;
mod state;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(plugins);
    }
}
