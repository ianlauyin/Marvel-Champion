use bevy::prelude::*;

use crate::{
    features::{
        cards::ModularSet,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionModularState;

pub struct CollectionModularCardListPlugin;

impl Plugin for CollectionModularCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionModularState::Cards), spawn_modular_cards)
            .add_systems(
                OnExit(CollectionModularState::Cards),
                clean_up::<ModularCardList>,
            );
    }
}

#[derive(Resource)]
pub struct CollectionModularSet(pub ModularSet);

#[derive(Component, Clone)]
struct ModularCardList;

fn spawn_modular_cards(
    commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionModularSet>,
) {
    let list_items = identity
        .0
        .get_cards()
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
        component: ModularCardList,
        previous_state: CollectionModularState::List,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
