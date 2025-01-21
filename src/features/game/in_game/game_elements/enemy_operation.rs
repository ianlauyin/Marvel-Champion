use crate::{
    features::{
        cards::{Card, Identity, ModularSet, Scenario},
        game::game_selector::{SelectedEncounterSet, SelectedPlayers},
    },
    utils::GameUtils,
};
use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::components::Belongs;

pub struct EnemyOperation;

impl EnemyOperation {
    pub fn init_game(
        mut commands: Commands,
        selected_players: &Res<SelectedPlayers>,
        selected_encounter_set: &Res<SelectedEncounterSet>,
    ) {
        let scenario = &selected_encounter_set.scenario;
        let modular_sets = &selected_encounter_set.modular_sets;
        init_main_scheme(commands.reborrow(), scenario);
        init_villain(commands.reborrow(), scenario, modular_sets);
        init_encounter_deck(
            commands.reborrow(),
            selected_encounter_set,
            selected_players.into_identities(),
        );
        init_nemesis_set(commands, selected_players.into_identities());
    }
}

fn init_villain(mut commands: Commands, scenario: &Scenario, modular_sets: &Vec<ModularSet>) {
    let villain_cards = if GameUtils::is_expert_mode(modular_sets) {
        scenario.get_expert_villain_cards()
    } else {
        scenario.get_standard_villain_cards()
    };
    for (index, villain_card) in villain_cards.iter().enumerate() {
        commands.spawn((villain_card.clone(), Belongs::Villain(index)));
    }
}

fn init_main_scheme(mut commands: Commands, scenario: &Scenario) {
    for (index, main_scheme_card) in scenario.get_main_scheme_cards().iter().enumerate() {
        commands.spawn((main_scheme_card.clone(), Belongs::MainScheme(index)));
    }
}

fn init_encounter_deck(
    mut commands: Commands,
    selected_encounter_set: &Res<SelectedEncounterSet>,
    identities: Vec<Identity>,
) {
    let mut encounter_deck: Vec<Card> = identities
        .iter()
        .map(|identity| identity.get_obligation())
        .collect();
    for modular_set in selected_encounter_set.modular_sets.iter() {
        encounter_deck.append(&mut modular_set.get_cards());
    }
    encounter_deck.append(&mut selected_encounter_set.scenario.get_encounter_cards());
    encounter_deck.shuffle(&mut rand::thread_rng());
    for (index, card) in encounter_deck.iter().enumerate() {
        commands.spawn((card.clone(), Belongs::EncounterDeck(index)));
    }
}

fn init_nemesis_set(mut commands: Commands, identities: Vec<Identity>) {
    let mut nemesis_set_cards: Vec<Card> = vec![];
    for identity in identities.iter() {
        nemesis_set_cards.append(&mut identity.get_nemesis_set());
    }
    for card in nemesis_set_cards {
        commands.spawn((card.clone(), Belongs::OutOfPlay));
    }
}
