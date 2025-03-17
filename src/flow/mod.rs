mod loading;
mod setup;
mod state;

use bevy::prelude::{App, Plugin};

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            setup::SetupPlugin,
            loading::LoadingPlugin,
            state::StatePlugin,
        ));
    }
}
