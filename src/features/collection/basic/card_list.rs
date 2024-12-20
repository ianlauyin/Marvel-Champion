use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, CardListBuilder, ListItem, MenuBuilder},
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

fn spawn_basic_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card_map = CardDatas::get_basic_cards()
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
        component: BasicCardList,
        previous_state: CollectionState::Menu,
        content_child,
    }
    .spawn(commands);
}
