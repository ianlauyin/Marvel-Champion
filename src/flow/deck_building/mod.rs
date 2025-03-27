mod hero_menu;
mod resource;
mod state;

use bevy::{
    app::{App, Plugin},
    state::app::AppExtStates,
};

pub struct DeckBuildingPlugin;

impl Plugin for DeckBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<state::DeckBuildingState>()
            .add_plugins((
                hero_menu::HeroMenuPlugin,
                resource::DeckBuildingResourcePlugin,
            ));
    }
}
