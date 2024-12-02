mod card_list;
mod list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionModularPlugin;

impl Plugin for CollectionModularPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            list::CollectionModularListPlugin,
            state::CollectionModularStatePlugin,
            card_list::CollectionModularCardListPlugin,
        ));
    }
}
