mod collection;
mod deck_building;
mod game;
mod loading;
mod main_menu;
mod setup;
mod state;

use bevy::prelude::*;

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::AppState>().add_plugins((
            setup::SetupPlugin,
            loading::LoadingPlugin,
            main_menu::MainMenuPlugin,
            collection::CollectionPlugin,
            deck_building::DeckBuildingPlugin,
            game::GamePlugin,
        ));
    }
}
