use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct ForcedResponseAbilities(Vec<fn(&mut World)>);
