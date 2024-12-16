use bevy::app::{App, Plugin};

pub mod cards;
mod collection;
mod deck_building;
mod game_mat;
mod main_menu;
pub mod shared;

pub struct FeaturePlugin;

impl Plugin for FeaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_mat::GameMatPlugin,
            cards::CardDataPlugin,
            main_menu::MainMenuPlugin,
            collection::CollectionPlugin,
            shared::ButtonUIPlugin,
            shared::ScrollingListPlugin,
            shared::CardDetailPlugin,
            deck_building::DeckBuildingPlugin,
            shared::PopupPlugin,
        ));
    }
}
