use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
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

fn spawn_leadership_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_leadership_cards()
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
        component: LeadershipCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
