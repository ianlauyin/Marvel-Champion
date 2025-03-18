use bevy::prelude::Component;

#[derive(Component)]
pub struct CardResources(Vec<CardResource>);

pub enum CardResource {
    Wild,
    Energy,
    Mental,
    Physical,
}
