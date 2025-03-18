use bevy::{
    ecs::{system::Resource, world::World},
    utils::HashMap,
};

use crate::component::card::CardBasic;

#[derive(Resource)]
pub struct CardDatas<'a>(HashMap<String, (CardBasic<'a>, fn(&mut World))>);

impl<'a> CardDatas<'a> {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        // Run through all belongs card datas
        Self(map)
    }
}
