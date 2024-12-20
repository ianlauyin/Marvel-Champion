use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, CardListBuilder, MenuBuilder},
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

fn spawn_justice_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card_map = CardDatas::get_justice_cards()
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
        component: JusticeCardList,
        previous_state: CollectionState::Menu,
        content_child,
    }
    .spawn(commands);
}
