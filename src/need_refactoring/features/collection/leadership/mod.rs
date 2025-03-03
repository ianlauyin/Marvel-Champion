mod card_list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionLeadershipPlugin;

impl Plugin for CollectionLeadershipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::CollectionLeadershipStatePlugin,
            card_list::CollectionLeadershipCardListPlugin,
        ));
    }
}
