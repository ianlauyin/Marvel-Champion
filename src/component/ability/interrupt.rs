use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct InterruptAbility {
    forced: bool,
    ability: fn(&mut World),
}
