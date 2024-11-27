mod card_list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionAggressionPlugin;

impl Plugin for CollectionAggressionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::CollectionAggressionStatePlugin,
            card_list::CollectionAggressionCardListPlugin,
        ));
    }
}
