use bevy::prelude::Component;

#[derive(Component)]
pub enum CardResource {
    Wild,
    Energy,
    Mental,
    Physical,
}