use bevy::prelude::{Component, World};

#[derive(Component)]
pub struct WhenRevealedAbility(fn(&mut World));
