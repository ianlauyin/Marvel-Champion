use bevy::prelude::Component;

#[derive(Component)]
pub struct CardResources(Vec<CardResource>);

impl CardResources {
    pub fn new(resources: Vec<CardResource>) -> Self {
        Self(resources)
    }

    pub fn wild() -> Self {
        Self(vec![CardResource::Wild])
    }

    pub fn energy() -> Self {
        Self(vec![CardResource::Energy])
    }

    pub fn mental() -> Self {
        Self(vec![CardResource::Mental])
    }

    pub fn physical() -> Self {
        Self(vec![CardResource::Physical])
    }
}

pub enum CardResource {
    Wild,
    Energy,
    Mental,
    Physical,
}
