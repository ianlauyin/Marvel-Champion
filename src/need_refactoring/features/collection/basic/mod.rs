mod card_list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionBasicPlugin;

impl Plugin for CollectionBasicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::CollectionBasicStatePlugin,
            card_list::CollectionBasicCardListPlugin,
        ));
    }
}
