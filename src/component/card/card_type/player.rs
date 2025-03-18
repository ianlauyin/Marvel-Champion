use bevy::prelude::Component;

#[derive(Component)]
pub enum PlayerCardType {
    Ally,
    Event,
    Resource,
    Support,
    Upgrade,
}
