use bevy::prelude::*;

use crate::features::{
    cards::{ModularSet, Villain},
    game::state::GameState,
    shared::{ListBuilder, ListItem, MenuBuilder, TextListBuilder, TextListItem},
};

use super::state::GameSelectorState;

pub struct GameSelectorEncounterPlugin;

const CURRENT_STATE: GameSelectorState = GameSelectorState::Encounter;

impl Plugin for GameSelectorEncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_encounter_list);
    }
}

#[derive(Component, Clone)]
struct EncounterList;

fn spawn_encounter_list(mut commands: Commands) {
    let villain_list = spawn_villlain_list(commands.reborrow());
    let modular_set_list = spawn_modular_list(commands.reborrow());

    let content_child = commands
        .spawn(Node::default())
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
