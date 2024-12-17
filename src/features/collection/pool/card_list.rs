use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionPoolState;

pub struct CollectionPoolCardListPlugin;

impl Plugin for CollectionPoolCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionPoolState::Cards), spawn_pool_cards)
            .add_systems(OnExit(CollectionPoolState::Cards), clean_up::<PoolCardList>);
    }
}

#[derive(Component, Clone)]
struct PoolCardList;

fn spawn_pool_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_pool_cards()
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
        component: PoolCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
