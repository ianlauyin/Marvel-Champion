use crate::features::cards::ModularSet;

pub struct GameUtils;

impl GameUtils {
    pub fn is_expert_mode(modular_sets: &Vec<ModularSet>) -> bool {
        modular_sets.contains(&ModularSet::Expert)
    }
}
