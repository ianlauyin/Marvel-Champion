use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionJusticeState;

pub struct CollectionJusticeCardListPlugin;

impl Plugin for CollectionJusticeCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionJusticeState::Cards), spawn_justice_cards)
            .add_systems(
                OnExit(CollectionJusticeState::Cards),
                clean_up::<JusticeCardList>,
            );
    }
}

#[derive(Component, Clone)]
struct JusticeCardList;

fn spawn_justice_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_justice_cards()
        .iter()
        .map(|card| {
            (
                CardDetailButton(card.clone()),
                ListItem {
                    image: ImageNode::new(asset_server.load(card.get_image_path())),
                    ..default()
                },
            )
        })
        .collect();
    MenuBuilder {
        component: JusticeCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
