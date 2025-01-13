use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct EncounterCard(usize);

impl EncounterCard {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}
