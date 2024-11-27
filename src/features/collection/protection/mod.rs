mod card_list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionProtectionPlugin;

impl Plugin for CollectionProtectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::CollectionProtectionStatePlugin,
            card_list::CollectionProtectionCardListPlugin,
        ));
    }
}
