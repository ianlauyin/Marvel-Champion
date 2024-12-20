use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, CardListBuilder, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionLeadershipState;

pub struct CollectionLeadershipCardListPlugin;

impl Plugin for CollectionLeadershipCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(CollectionLeadershipState::Cards),
            spawn_leadership_cards,
        )
        .add_systems(
            OnExit(CollectionLeadershipState::Cards),
            clean_up::<LeadershipCardList>,
        );
    }
}

#[derive(Component, Clone)]
struct LeadershipCardList;

fn spawn_leadership_cards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card_map = CardDatas::get_leadership_cards()
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
        component: LeadershipCardList,
        previous_state: CollectionState::Menu,
        content_child,
    }
    .spawn(commands);
}
