use bevy::prelude::*;

use crate::{
    features::{
        cards::Villain,
        shared::{CardDetailButton, CardListBuilder, MenuBuilder},
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionVillainSet>,
) {
    let card_map = identity
        .0
        .get_cards()
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
        component: VillainCardList,
        previous_state: CollectionVillainState::List,
        content_child,
    }
    .spawn(commands);
}
