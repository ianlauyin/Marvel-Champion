use crate::features::{
    cards::{Card, Identity, ModularSet, Villain},
    game::game_selector::{SelectedEncounterSet, SelectedPlayers},
};
use bevy::prelude::*;
use rand::seq::SliceRandom;

use super::components::{CardState, EncounterCard, OutOfPlayArea};

pub struct EnemyOperation;

impl EnemyOperation {
    pub fn init_game(
        mut commands: Commands,
        selected_players: &Res<SelectedPlayers>,
        selected_encounter_set: &Res<SelectedEncounterSet>,
    ) {
        let villain = &selected_encounter_set.villain;
        let modular_sets = &selected_encounter_set.modular_sets;
        Self::init_villain(commands.reborrow(), villain, modular_sets);
        Self::init_main_scheme(commands.reborrow(), villain);
        Self::init_encounter_deck(
            commands.reborrow(),
            selected_encounter_set,
            selected_players.into_identities(),
        );
        Self::init_nemesis_set(commands, selected_players.into_identities());
    }

    fn init_villain(mut commands: Commands, villain: &Villain, modular_sets: &Vec<ModularSet>) {
        let villain_cards = if Self::is_expert_mode(modular_sets) {
            villain.get_expert_villain_cards()
        } else {
            villain.get_standard_villain_cards()
        };
        let in_play_villain = villain_cards.first().unwrap();
        commands.spawn((in_play_villain.clone(), CardState::InPlay));
        for out_play_villain in villain_cards.iter().skip(1) {
            commands.spawn((out_play_villain.clone(), CardState::OutPlay));
        }
    }

    fn init_main_scheme(mut commands: Commands, villain: &Villain) {
        let main_scheme_cards = villain.get_main_scheme_cards();
        let in_play_main_scheme = main_scheme_cards.first().unwrap();
        commands.spawn((in_play_main_scheme.clone(), CardState::InPlay));
        for out_play_main_scheme in main_scheme_cards.iter().skip(1) {
            commands.spawn((out_play_main_scheme.clone(), CardState::OutPlay));
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
        encounter_deck.append(&mut selected_encounter_set.villain.get_encounter_cards());
        encounter_deck.shuffle(&mut rand::thread_rng());
        for (index, card) in encounter_deck.iter().enumerate() {
            commands.spawn((card.clone(), EncounterCard::new(index), CardState::OutPlay));
        }
    }

    fn init_nemesis_set(mut commands: Commands, identities: Vec<Identity>) {
        let mut nemesis_set_cards: Vec<Card> = vec![];
        for identity in identities.iter() {
            nemesis_set_cards.append(&mut identity.get_nemesis_set());
        }
        for card in nemesis_set_cards {
            commands.spawn((card.clone(), CardState::OutPlay, OutOfPlayArea));
        }
    }

    fn is_expert_mode(modular_sets: &Vec<ModularSet>) -> bool {
        modular_sets.contains(&ModularSet::Expert)
    }
}
