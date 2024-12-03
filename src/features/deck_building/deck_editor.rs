use bevy::prelude::*;

#[derive(Resource)]
pub struct EditingDeck(pub Option<usize>);
