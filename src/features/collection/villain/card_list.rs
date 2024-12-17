use bevy::prelude::*;

use crate::{
    features::{
        cards::Villain,
        shared::{CardDetailButton, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionVillainState;

pub struct CollectionVillainCardListPlugin;

impl Plugin for CollectionVillainCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionVillainState::Cards), spawn_villain_cards)
            .add_systems(
                OnExit(CollectionVillainState::Cards),
                clean_up::<VillainCardList>,
            );
    }
}

#[derive(Resource)]
pub struct CollectionVillainSet(pub Villain);

#[derive(Component, Clone)]
struct VillainCardList;

fn spawn_villain_cards(
    commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionVillainSet>,
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
        component: VillainCardList,
        previous_state: CollectionVillainState::List,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}
