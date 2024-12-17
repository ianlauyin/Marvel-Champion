use bevy::prelude::*;

use crate::{
    features::{
        cards::CardDatas,
        collection::state::CollectionState,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionAggressionState;

pub struct CollectionAggressionCardListPlugin;

impl Plugin for CollectionAggressionCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(CollectionAggressionState::Cards),
            spawn_aggression_cards,
        )
        .add_systems(
            OnExit(CollectionAggressionState::Cards),
            clean_up::<AggressionCardList>,
        );
    }
}

#[derive(Component, Clone)]
struct AggressionCardList;

fn spawn_aggression_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_aggression_cards()
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
        component: AggressionCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
