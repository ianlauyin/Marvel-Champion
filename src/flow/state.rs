use bevy::prelude::*;

#[derive(States, Default, Hash, PartialEq, Eq, Debug, Clone)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Game,
    DeckBuilding,
    Collection,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
    }
}
