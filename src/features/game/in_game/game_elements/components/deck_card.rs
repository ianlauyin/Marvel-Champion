use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct DeckCard(usize);

impl DeckCard {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}
