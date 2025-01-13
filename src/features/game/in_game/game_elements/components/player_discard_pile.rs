use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct PlayerDiscardPile(usize);

impl PlayerDiscardPile {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}
