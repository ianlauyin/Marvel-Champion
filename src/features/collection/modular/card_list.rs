use bevy::prelude::*;

use crate::{
    features::{
        cards::ModularSet,
        shared::{CardDetailButton, CardListBuilder, MenuBuilder},
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionModularSet>,
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
        component: ModularCardList,
        previous_state: CollectionModularState::List,
        content_child,
    }
    .spawn(commands);
}
