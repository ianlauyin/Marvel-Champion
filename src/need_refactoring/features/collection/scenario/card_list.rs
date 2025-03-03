use bevy::prelude::*;

use crate::{
    features::{
        cards::Scenario,
        shared::{CardDetailButton, CardListBuilder, MenuBuilder},
    },
    systems::clean_up,
};

use super::state::CollectionScenarioState;

pub struct CollectionScenarioCardListPlugin;

impl Plugin for CollectionScenarioCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(CollectionScenarioState::Cards),
            spawn_scenario_cards,
        )
        .add_systems(
            OnExit(CollectionScenarioState::Cards),
            clean_up::<ScenarioCardList>,
        );
    }
}

#[derive(Resource)]
pub struct CollectionScenarioSet(pub Scenario);

#[derive(Component, Clone)]
struct ScenarioCardList;

fn spawn_scenario_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionScenarioSet>,
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
        next_state: None::<CollectionScenarioState>,
        component: ScenarioCardList,
        previous_state: CollectionScenarioState::List,
        content_child,
    }
    .spawn(commands);
}
