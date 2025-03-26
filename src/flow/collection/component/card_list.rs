use bevy::prelude::*;

use crate::cards::SetTrait;

#[derive(Component)]
pub struct CardList(pub Box<dyn SetTrait>);

pub struct CardListPlugin;

impl Plugin for CardListPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup_card_list);
    }
}
