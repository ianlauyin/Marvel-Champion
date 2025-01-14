use bevy::prelude::*;

use crate::{
    features::{
        cards::Scenario,
        collection::state::CollectionState,
        shared::{ListBuilder, ListItem, MenuBuilder},
    },
    systems::{clean_up, LoadAsset},
};

use super::{card_list::CollectionScenarioSet, state::CollectionScenarioState};

pub struct CollectionScenarioListPlugin;

impl Plugin for CollectionScenarioListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Scenario), spawn_scenario_list)
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CollectionScenarioState::List)),
            )
            .add_systems(OnExit(CollectionState::Scenario), clean_up::<ScenarioList>);
    }
}

#[derive(Component, Clone)]
struct ScenarioList;

fn spawn_scenario_list(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scenarios = Scenario::get_all();
    let list_map = scenarios
        .iter()
        .map(|scenario| {
            (
                scenario.clone(),
                ListItem {
                    text: scenario.to_string().clone(),
                    image: ImageNode::new(asset_server.load(scenario.get_title_image_path()))
                        .with_color(Color::srgb(0.365, 0.365, 0.365)),
                    ..default()
                },
            )
        })
        .collect();
    let content_child = ListBuilder(list_map).spawn(commands.reborrow());
    MenuBuilder {
        next_state: None::<CollectionState>,
        component: ScenarioList,
        previous_state: CollectionState::Menu,
        content_child,
    }
    .spawn(commands);
}

fn handle_button_interaction(
    mut commands: Commands,
    button_q: Query<(&Interaction, &Scenario), With<Button>>,
    mut next_state: ResMut<NextState<CollectionScenarioState>>,
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, scenario) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            for card in scenario.get_cards() {
                load_asset.add_card(&card, &asset_server);
            }
            commands.insert_resource(CollectionScenarioSet(scenario.clone()));
            next_state.set(CollectionScenarioState::LoadingCards);
            return;
        }
    }
}
