mod card_list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionPoolPlugin;

impl Plugin for CollectionPoolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::CollectionPoolStatePlugin,
            card_list::CollectionPoolCardListPlugin,
        ));
    }
}
