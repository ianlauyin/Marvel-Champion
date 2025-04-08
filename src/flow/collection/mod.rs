mod component;
mod menu;

use bevy::prelude::{App, Plugin};

use super::state::AppState;

const CURRENT_STATE: AppState = AppState::Collection;

pub struct CollectionPlugin;

impl Plugin for CollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu::CollectionMenuPlugin,
            component::CollectionComponentsPlugin,
        ));
    }
}
