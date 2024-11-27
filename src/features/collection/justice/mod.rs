mod card_list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionJusticePlugin;

impl Plugin for CollectionJusticePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::CollectionJusticeStatePlugin,
            card_list::CollectionJusticeCardListPlugin,
        ));
    }
}
