mod card_list;
mod list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionVillainPlugin;

impl Plugin for CollectionVillainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            list::CollectionVillainListPlugin,
            state::CollectionVillainStatePlugin,
            card_list::CollectionVillainCardListPlugin,
        ));
    }
}
