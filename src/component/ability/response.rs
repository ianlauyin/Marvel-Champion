use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct ResponseAbility {
    forced: bool,
    ability: fn(&mut World),
}
