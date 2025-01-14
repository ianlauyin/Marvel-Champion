use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{ModularSet, Scenario},
        game::state::GameState,
        shared::{MenuBuilder, NextButton, Popup, TextListBuilder, TextListItem},
    },
    systems::clean_up,
};

use super::{identity::SelectedPlayers, state::GameSelectorState};

pub struct GameSelectorEncounterPlugin;

const CURRENT_STATE: GameSelectorState = GameSelectorState::Encounter;

impl Plugin for GameSelectorEncounterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedEncounterSet {
            scenario: Scenario::CoreRhino,
            modular_sets: vec![ModularSet::Standard],
        })
        .add_systems(OnEnter(CURRENT_STATE), spawn_encounter_list)
        .add_systems(
            Update,
            (
                handle_scenario_item_interaction,
                handle_modular_set_item_interaction,
            )
                .run_if(in_state(CURRENT_STATE))
                .run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(
            Update,
            (handle_ui_change, handle_next_state).run_if(in_state(CURRENT_STATE)),
        )
        .add_systems(OnExit(CURRENT_STATE), clean_up::<EncounterList>);
    }
}

#[derive(Resource)]
pub struct SelectedEncounterSet {
    pub scenario: Scenario,
    pub modular_sets: Vec<ModularSet>,
}

#[derive(Component, Clone)]
struct EncounterList;

fn spawn_encounter_list(mut commands: Commands) {
    let scenario_list = spawn_villlain_list(commands.reborrow());
    let modular_set_list = spawn_modular_list(commands.reborrow());

    let content_child = commands
        .spawn(Node {
            height: Val::Percent(100.),
            ..default()
        })
        .add_children(&[scenario_list, modular_set_list])
        .id();

    MenuBuilder {
        next_state: Some(GameState::InGame),
        component: EncounterList,
        previous_state: GameSelectorState::Identity,
        content_child,
    }
    .spawn(commands);
}

#[derive(Component, Clone)]
struct VillainItem(Scenario);

fn spawn_villlain_list(mut commands: Commands) -> Entity {
    let scenario = Scenario::get_all();
    let list_map = scenario
        .iter()
        .map(|scenario| {
            (
                VillainItem(scenario.clone()),
                TextListItem {
                    color: Color::NONE,
                    text: scenario.to_string().clone(),
                    ..default()
                },
            )
        })
        .collect();

    TextListBuilder(list_map).spawn(commands.reborrow())
}

#[derive(Component, Clone)]
struct ModularSetItem(ModularSet);

fn spawn_modular_list(mut commands: Commands) -> Entity {
    let modular_sets = ModularSet::get_all();
    let list_map = modular_sets
        .iter()
        .map(|modular_set| {
            (
                ModularSetItem(modular_set.clone()),
                TextListItem {
                    text: modular_set.to_string().clone(),
                    color: Color::NONE,
                },
            )
        })
        .collect();

    TextListBuilder(list_map).spawn(commands.reborrow())
}

fn handle_scenario_item_interaction(
    text_q: Query<(&Interaction, &VillainItem)>,
    mut selected_encounter_set: ResMut<SelectedEncounterSet>,
) {
    for (interaction, item) in text_q.iter() {
        if *interaction == Interaction::Pressed {
            selected_encounter_set.scenario = item.0.clone();
        }
    }
}

fn handle_modular_set_item_interaction(
    text_q: Query<(&Interaction, &ModularSetItem)>,
    mut selected_encounter_set: ResMut<SelectedEncounterSet>,
) {
    for (interaction, item) in text_q.iter() {
        if *interaction == Interaction::Pressed {
            if selected_encounter_set.modular_sets.contains(&item.0) {
                selected_encounter_set
                    .modular_sets
                    .retain(|modular_set| modular_set != &item.0);
                return;
            }
            selected_encounter_set.modular_sets.push(item.0.clone());
            return;
        }
    }
}

fn handle_ui_change(
    mut scenario_item_q: Query<(&mut BorderColor, &VillainItem), Without<ModularSetItem>>,
    mut modular_set_item_q: Query<(&mut BorderColor, &ModularSetItem), Without<VillainItem>>,
    selected_encounter_set: ResMut<SelectedEncounterSet>,
) {
    if selected_encounter_set.is_changed() {
        for (mut scenario_border_color, scenario_item) in scenario_item_q.iter_mut() {
            if selected_encounter_set.scenario == scenario_item.0 {
                scenario_border_color.0 = Color::from(Color::WHITE);
            } else {
                scenario_border_color.0 = Color::from(Color::NONE);
            }
        }
        for (mut modular_set_border_color, modular_set_item) in modular_set_item_q.iter_mut() {
            if selected_encounter_set
                .modular_sets
                .contains(&modular_set_item.0)
            {
                modular_set_border_color.0 = Color::from(Color::WHITE);
            } else {
                modular_set_border_color.0 = Color::from(Color::NONE);
            }
        }
    }
}

fn handle_next_state(
    mut commands: Commands,
    next_button_q: Query<(&Interaction, &NextButton<GameState>)>,
    mut next_state: ResMut<NextState<GameState>>,
    selected_encounter_set: Res<SelectedEncounterSet>,
    selected_players: Res<SelectedPlayers>,
) {
    let Ok((interaction, next_button)) = next_button_q.get_single() else {
        warn!("NextButton<GameState> not found");
        return;
    };
    if *interaction == Interaction::Pressed {
        if let Err(message) = validate_resource(selected_encounter_set, selected_players) {
            commands.spawn(Popup::new(message));
            return;
        }
        next_state.set(next_button.0.clone());
    }
}

fn validate_resource(
    selected_encounter_set: Res<SelectedEncounterSet>,
    selected_players: Res<SelectedPlayers>,
) -> Result<(), String> {
    if selected_players.0.is_empty() {
        return Err("You must select at least 1 player.".to_string());
    }
    if selected_encounter_set.modular_sets.is_empty() {
        return Err("You must select at least 1 modular set.".to_string());
    }
    if !selected_encounter_set
        .modular_sets
        .contains(&ModularSet::Standard)
    {
        return Err("You must select the standard modular set.".to_string());
    }
    let modular_for_check = selected_encounter_set
        .modular_sets
        .iter()
        .filter(|modular_set| ![ModularSet::Standard, ModularSet::Expert].contains(modular_set));
    if modular_for_check.count() != selected_encounter_set.scenario.get_encounter_set_numbers() {
        return Err(format!(
            "You must only select {} modular set(s). (Standard and Expert not included)",
            selected_encounter_set.scenario.get_encounter_set_numbers()
        ));
    }
    Ok(())
}
