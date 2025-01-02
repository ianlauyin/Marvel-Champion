use std::collections::HashSet;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{ModularSet, Villain},
        game::state::GameState,
        shared::{MenuBuilder, TextListBuilder, TextListItem},
    },
    systems::clean_up,
};

use super::state::GameSelectorState;

pub struct GameSelectorEncounterPlugin;

const CURRENT_STATE: GameSelectorState = GameSelectorState::Encounter;

impl Plugin for GameSelectorEncounterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedEncounterSet>()
            .add_systems(OnEnter(CURRENT_STATE), spawn_encounter_list)
            .add_systems(
                Update,
                (
                    handle_villain_item_interaction,
                    handle_modular_set_item_interaction,
                )
                    .run_if(in_state(CURRENT_STATE))
                    .run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_systems(Update, handle_ui_change)
            .add_systems(OnExit(CURRENT_STATE), clean_up::<EncounterList>);
    }
}

#[derive(Resource, Default)]
pub struct SelectedEncounterSet {
    pub villain: Option<Villain>,
    pub modular_set: Vec<ModularSet>,
}

#[derive(Component, Clone)]
struct EncounterList;

fn spawn_encounter_list(mut commands: Commands) {
    let villain_list = spawn_villlain_list(commands.reborrow());
    let modular_set_list = spawn_modular_list(commands.reborrow());

    let content_child = commands
        .spawn(Node {
            height: Val::Percent(100.),
            ..default()
        })
        .add_children(&[villain_list, modular_set_list])
        .id();

    MenuBuilder {
        next_state: None::<GameState>,
        component: EncounterList,
        previous_state: GameSelectorState::Identity,
        content_child,
    }
    .spawn(commands);
}

#[derive(Component, Clone)]
struct VillainList;

#[derive(Component, Clone)]
struct VillainItem(Villain);

fn spawn_villlain_list(mut commands: Commands) -> Entity {
    let villain = Villain::get_all();
    let list_map = villain
        .iter()
        .map(|villain| {
            (
                VillainItem(villain.clone()),
                TextListItem {
                    color: Color::NONE,
                    text: villain.to_string().clone(),
                    ..default()
                },
            )
        })
        .collect();

    TextListBuilder(list_map).spawn(commands.reborrow())
}

#[derive(Component, Clone)]
struct ModularSetList;

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

fn handle_villain_item_interaction(
    text_q: Query<(&Interaction, &VillainItem)>,
    mut selected_encounter_set: ResMut<SelectedEncounterSet>,
) {
    for (interaction, item) in text_q.iter() {
        if *interaction == Interaction::Pressed {
            selected_encounter_set.villain = Some(item.0.clone());
        }
    }
}

fn handle_modular_set_item_interaction(
    text_q: Query<(&Interaction, &ModularSetItem)>,
    mut selected_encounter_set: ResMut<SelectedEncounterSet>,
) {
    for (interaction, item) in text_q.iter() {
        if *interaction == Interaction::Pressed {
            if selected_encounter_set.modular_set.contains(&item.0) {
                selected_encounter_set
                    .modular_set
                    .retain(|modular_set| modular_set != &item.0);
                return;
            }
            selected_encounter_set.modular_set.push(item.0.clone());
            return;
        }
    }
}

fn handle_ui_change(
    mut villain_item_q: Query<(&mut BorderColor, &VillainItem), Without<ModularSetItem>>,
    mut modular_set_item_q: Query<(&mut BorderColor, &ModularSetItem), Without<VillainItem>>,
    selected_encounter_set: ResMut<SelectedEncounterSet>,
) {
    if selected_encounter_set.is_changed() {
        for (mut villain_border_color, villain_item) in villain_item_q.iter_mut() {
            if let Some(villain) = &selected_encounter_set.villain {
                if *villain == villain_item.0 {
                    villain_border_color.0 = Color::from(Color::WHITE);
                } else {
                    villain_border_color.0 = Color::from(Color::NONE);
                }
            } else {
                villain_border_color.0 = Color::from(Color::NONE);
            }
        }
        for (mut modular_set_border_color, modular_set_item) in modular_set_item_q.iter_mut() {
            if selected_encounter_set
                .modular_set
                .contains(&modular_set_item.0)
            {
                modular_set_border_color.0 = Color::from(Color::WHITE);
            } else {
                modular_set_border_color.0 = Color::from(Color::NONE);
            }
        }
    }
}
