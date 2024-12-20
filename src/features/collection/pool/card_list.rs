use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, CardListBuilder, MenuBuilder},
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

fn spawn_pool_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card_map = CardDatas::get_pool_cards()
        .iter()
        .map(|card| {
            (
                CardDetailButton(card.clone()),
                ImageNode::new(asset_server.load(card.get_image_path())),
            )
        })
        .collect();

    let content_child = CardListBuilder {
        card_map,
        card_size: (Val::Px(128.), Val::Px(178.)),
        height: Val::Percent(90.),
        columns: 8,
    }
    .spawn(commands.reborrow());
    MenuBuilder {
        component: PoolCardList,
        previous_state: CollectionState::Menu,
        content_child,
    }
    .spawn(commands);
}
