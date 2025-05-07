use bevy::ecs::resource::Resource;

use crate::cards::{ModularSet, Scenario};

#[derive(Resource, Default)]
pub struct EnemyInfo {
    scenario: Option<Scenario>,
    modular_sets: Vec<ModularSet>,
    expert_mode: bool,
}

impl EnemyInfo {
    pub fn select_scenario(&mut self, scenario: &Scenario) {
        self.scenario = Some(scenario.clone());
    }

    pub fn select_modular_set(&mut self, modular_set: &ModularSet) {
        self.modular_sets.push(modular_set.clone());
    }

    pub fn remove_modular_set(&mut self, modular_set: &ModularSet) {
        if let Some(index) = self.modular_sets.iter().position(|set| set == modular_set) {
            self.modular_sets.remove(index);
        }
    }

    pub fn set_expert_mode(&mut self, is_expert: bool) {
        self.expert_mode = is_expert;
    }
}
