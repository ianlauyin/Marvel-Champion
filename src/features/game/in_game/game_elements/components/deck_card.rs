use bevy::prelude::*;


// TODO: Add plugin for handling add will trigger CardState change
#[derive(Component, Clone)]
pub struct DeckCard(usize);

impl DeckCard {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}
