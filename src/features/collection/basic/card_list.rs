use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionBasicState;

pub struct CollectionBasicCardListPlugin;

impl Plugin for CollectionBasicCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionBasicState::Cards), spawn_basic_cards)
            .add_systems(
                OnExit(CollectionBasicState::Cards),
                clean_up::<BasicCardList>,
            );
    }
}

#[derive(Component, Clone)]
struct BasicCardList;

fn spawn_basic_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_basic_cards()
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
        component: BasicCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
