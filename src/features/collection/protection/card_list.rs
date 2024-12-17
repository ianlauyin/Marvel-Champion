use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionProtectionState;

pub struct CollectionProtectionCardListPlugin;

impl Plugin for CollectionProtectionCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(CollectionProtectionState::Cards),
            spawn_protection_cards,
        )
        .add_systems(
            OnExit(CollectionProtectionState::Cards),
            clean_up::<ProtectionCardList>,
        );
    }
}

#[derive(Component, Clone)]
struct ProtectionCardList;

fn spawn_protection_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_protection_cards()
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
        component: ProtectionCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
