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
